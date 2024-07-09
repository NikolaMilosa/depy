use std::path::{Path, PathBuf};

use cargo_toml::{Manifest, Package};

use crate::model::{Dep, Target, TargetKind};

use super::ConfigParser;

#[derive(Debug, Clone, Default)]
pub struct RustConfiguration {}

impl ConfigParser for RustConfiguration {
    fn parse(&self, path: PathBuf) -> anyhow::Result<Vec<Target>> {
        let file = std::fs::read(&path)?;
        let mut manifest = Manifest::from_slice(&file)?;
        manifest.complete_from_path(&path)?;
        let mut targets = vec![];
        if let Some(ref package) = manifest.package {
            let target = Self::build_target_from_package(package, &manifest)?;
            targets.push(target);
        } else {
            targets.extend(self.build_targets_from_workspace(manifest, path)?);
        }
        Ok(targets)
    }

    fn matches(&self, path: &Path) -> bool {
        path.ends_with("Cargo.toml")
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
                Target::new(key.to_string(), kind, version)
            })
            .filter(|t| t.kind != TargetKind::Crate)
            .map(|d| Dep {
                name: d.name,
                kind: d.kind,
            })
            .collect();

        target.add_dependencies(deps);

        Ok(target)
    }

    fn build_targets_from_workspace(
        &self,
        manifest: Manifest,
        path: PathBuf,
    ) -> anyhow::Result<Vec<Target>> {
        let mut targets = vec![];
        let workspace = manifest
            .workspace
            .ok_or(anyhow::anyhow!("Workspace not found"))?;
        let package = workspace.package.unwrap_or_default();
        let version = package.version.unwrap_or_default();

        for member in &workspace.members {
            let current_path = &path.parent().unwrap().join(member).join("Cargo.toml");
            let member_targets: Vec<_> =
                Self::parse(&RustConfiguration {}, current_path.to_owned())?
                    .iter_mut()
                    .map(|t| {
                        t.version = version.to_string();
                        t.clone()
                    })
                    .collect();
            targets.extend(member_targets);
        }

        Ok(targets)
    }
}
