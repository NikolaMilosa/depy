#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Target {
    pub name: String,
    pub kind: TargetKind,
    pub version: String,
    pub dependencies: Vec<String>,
    pub height: usize,
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.kind == other.kind && self.version == other.version
    }
}
impl Target {
    pub fn new(name: String, kind: TargetKind, version: String) -> Self {
        Self {
            name,
            kind,
            version,
            dependencies: vec![],
            height: 0,
        }
    }

    pub fn add_dependencies(&mut self, dependencies: Vec<String>) {
        self.dependencies.extend(dependencies);
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
            match &self.dependencies.is_empty() {
                true => ":",
                false => "",
            }
        )?;

        for dependency in &self.dependencies {
            write!(f, " |- {}", dependency)?;
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

impl Display for TargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetKind::Binary => write!(f, "bin "),
            TargetKind::Library => write!(f, "lib "),
            TargetKind::Crate => write!(f, ""),
        }
    }
}
