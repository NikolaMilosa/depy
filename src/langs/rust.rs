use std::{borrow::BorrowMut, collections::BTreeMap, path::PathBuf};

use cargo_toml::{Dependency, Manifest, Package};
use clap::Args;

use crate::model::{Target, TargetKind};

use super::ConfigParser;

#[derive(Debug, Clone, Args)]
pub struct RustConfiguration {
    /// Path to top level Cargo.toml
    pub path: PathBuf,
}

impl ConfigParser for RustConfiguration {
    fn parse(&self) -> anyhow::Result<Vec<Target>> {
        let file = std::fs::read(&self.path)?;
        let mut manifest = Manifest::from_slice(&file)?;
        manifest.complete_from_path(&self.path)?;
        let mut targets = vec![];
        if let Some(ref package) = manifest.package {
            let target = Self::build_target_from_package(package, &manifest)?;
            targets.push(target);
        } else {
            targets.extend(self.build_targets_from_workspace(manifest)?);
        }
        Ok(targets)
    }
}

impl RustConfiguration {
    fn build_target_from_package(package: &Package, manifest: &Manifest) -> anyhow::Result<Target> {
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
                let (version, kind) = match value {
                    cargo_toml::Dependency::Simple(s) => (s.to_string(), TargetKind::Crate),
                    cargo_toml::Dependency::Inherited(_) => ("".to_string(), TargetKind::Crate),
                    cargo_toml::Dependency::Detailed(d) => (
                        d.version
                            .clone()
                            .unwrap_or(d.rev.clone().unwrap_or_default()),
                        match d.path {
                            Some(_) => TargetKind::Library,
                            None => TargetKind::Crate,
                        },
                    ),
                };
                Target::new(key.clone(), kind, version)
            })
            .collect();

        target.add_dependencies(deps);

        Ok(target)
    }

    fn build_targets_from_workspace(&self, manifest: Manifest) -> anyhow::Result<Vec<Target>> {
        let mut targets = vec![];
        let workspace = manifest
            .workspace
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        let package = workspace.package.unwrap_or_default();
        let version = package.version.unwrap_or_default();
        let workspace_deps: BTreeMap<String, Dependency> = workspace
            .dependencies
            .into_iter()
            .chain(manifest.build_dependencies)
            .chain(manifest.dev_dependencies)
            .collect();
        for member in &workspace.members {
            let current_path = self.path.parent().unwrap().join(member).join("Cargo.toml");
            let member_targets: Vec<_> = Self::parse(&RustConfiguration { path: current_path })?
                .iter_mut()
                .map(|t| {
                    t.version = version.to_string();
                    if let Some(deps) = t.dependencies.borrow_mut() {
                        deps.iter_mut().for_each(|d| {
                            if d.version.eq("") {
                                if workspace_deps.contains_key(&d.name) {
                                    let dep = workspace_deps.get(&d.name).unwrap();
                                    d.version = match dep {
                                        Dependency::Simple(s) => s.to_string(),
                                        Dependency::Inherited(_) => version.to_string(),
                                        Dependency::Detailed(s) => {
                                            s.version.clone().unwrap_or(version.to_string())
                                        }
                                    };
                                } else if d.kind == TargetKind::Library {
                                    d.version = version.to_string();
                                }
                            }
                        });
                    }
                    t.clone()
                })
                .collect();
            targets.extend(member_targets);
        }

        Ok(targets)
    }
}
