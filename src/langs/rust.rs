use std::{borrow::BorrowMut, collections::BTreeMap, path::PathBuf};

use cargo_toml::{Dependency, Manifest};
use clap::Args;

use crate::model::{Target, TargetKind};

#[derive(Debug, Clone, Args)]
pub struct RustConfiguration {
    pub path: PathBuf,
}

impl RustConfiguration {
    pub fn parse(&self) -> anyhow::Result<Vec<Target>> {
        let file = std::fs::read(&self.path)?;
        let mut manifest = Manifest::from_slice(&file)?;
        manifest.complete_from_path(&self.path)?;
        let mut targets = vec![];
        if let Some(ref package) = manifest.package {
            let mut target = Target::new(
                package.name.clone(),
                match manifest.lib.is_none() {
                    true => TargetKind::Binary,
                    false => TargetKind::Library,
                },
                match manifest.needs_workspace_inheritance() {
                    true => "".to_string(),
                    false => package.version().to_string(),
                },
            );

            let deps = manifest
                .dependencies
                .iter()
                .chain(manifest.dev_dependencies.iter())
                .chain(manifest.build_dependencies.iter())
                .map(|(key, value)| {
                    Target::new(key.clone(), TargetKind::Library, {
                        match value {
                            cargo_toml::Dependency::Simple(s) => s.to_string(),
                            cargo_toml::Dependency::Inherited(_) => "".to_string(),
                            cargo_toml::Dependency::Detailed(d) => {
                                d.version.clone().unwrap_or_default()
                            }
                        }
                    })
                })
                .collect();

            target.add_dependencies(deps);

            targets.push(target);
        } else {
            let workspace = manifest
                .workspace
                .ok_or(anyhow::anyhow!("Workspace not found"))?;
            let package = workspace.package.unwrap_or_default();
            let version = package.version.unwrap_or_default();
            let workspace_deps: BTreeMap<String, Dependency> = workspace
                .dependencies
                .into_iter()
                .chain(manifest.build_dependencies.into_iter())
                .chain(manifest.dev_dependencies.into_iter())
                .collect();
            for member in workspace.members {
                let current_path = self.path.parent().unwrap().join(&member).join("Cargo.toml");
                let member_targets: Vec<_> =
                    Self::parse(&RustConfiguration { path: current_path })?
                        .iter_mut()
                        .map(|t| {
                            t.version = version.to_string();
                            if let Some(deps) = t.dependencies.borrow_mut() {
                                deps.iter_mut().for_each(|d| {
                                    if d.version.eq("") && workspace_deps.contains_key(&d.name) {
                                        let version = workspace_deps.get(&d.name).unwrap();
                                        d.version = match version {
                                            Dependency::Simple(s) => s.to_string(),
                                            Dependency::Inherited(_) => "".to_string(),
                                            Dependency::Detailed(s) => {
                                                s.version.clone().unwrap_or_default()
                                            }
                                        };
                                    }
                                });
                            }
                            t.clone()
                        })
                        .collect();
                targets.extend(member_targets);
            }
        }
        Ok(targets)
    }
}
