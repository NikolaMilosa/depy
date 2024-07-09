#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Target {
    pub name: String,
    pub kind: TargetKind,
    pub version: String,
    pub dependencies: Option<Vec<Dep>>,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct Dep {
    pub name: String,
    pub kind: TargetKind,
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.kind == other.kind && self.version == other.version
    }
}

impl PartialEq<Dep> for Target {
    fn eq(&self, other: &Dep) -> bool {
        self.name == other.name && self.kind == other.kind
    }
}

impl Target {
    pub fn new(name: String, kind: TargetKind, version: String) -> Self {
        Self {
            name,
            kind,
            version,
            dependencies: None,
            height: 0,
        }
    }

    pub fn add_dependencies(&mut self, dependencies: Vec<Dep>) {
        let mut old = self.dependencies.take().unwrap_or_default();
        old.extend(dependencies);
        self.dependencies = Some(old);
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "({}) {}{} {}{}",
            self.height,
            self.kind,
            self.name,
            self.version,
            match &self.dependencies {
                Some(_) => ":",
                None => "",
            }
        )?;

        if let Some(dependencies) = &self.dependencies {
            for dependency in dependencies {
                writeln!(f, " |- {}", dependency)?;
            }
        }

        Ok(())
    }
}

impl Display for Dep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetKind {
    Binary,
    Library,
    Crate,
}

impl Display for TargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetKind::Binary => write!(f, "bin "),
            TargetKind::Library => write!(f, "lib "),
            TargetKind::Crate => write!(f, ""),
        }
    }
}
