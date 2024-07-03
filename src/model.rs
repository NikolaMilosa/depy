#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Target {
    pub name: String,
    pub kind: TargetKind,
    pub version: String,
    pub dependencies: Option<Vec<Target>>,
}

impl Target {
    pub fn new(name: String, kind: TargetKind, version: String) -> Self {
        Self {
            name,
            kind,
            version,
            dependencies: None,
        }
    }

    pub fn add_dependencies(&mut self, dependencies: Vec<Target>) {
        let mut old_deps = self.dependencies.take().unwrap_or_default();
        old_deps.extend(dependencies);
        self.dependencies = Some(old_deps);
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}{}\n",
            self.kind.to_string(),
            self.name,
            self.version,
            match &self.dependencies {
                Some(_) => ":",
                None => "",
            }
        )?;

        if let Some(dependencies) = &self.dependencies {
            for dependency in dependencies {
                write!(f, " |- {}", dependency)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetKind {
    Binary,
    Library,
    Crate,
}

impl ToString for TargetKind {
    fn to_string(&self) -> String {
        match self {
            TargetKind::Binary => "bin ",
            TargetKind::Library => "lib ",
            TargetKind::Crate => "",
        }
        .to_string()
    }
}
