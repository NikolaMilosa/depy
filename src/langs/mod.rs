use std::path::{Path, PathBuf};

use crate::model::Target;
use csharp::CSharpConfiguration;
use rust::RustConfiguration;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod csharp;
mod rust;

#[derive(Debug, EnumIter)]
pub enum LanguagesConfiguration {
    Rust(RustConfiguration),
    CSharp(CSharpConfiguration),
}

impl Default for LanguagesConfiguration {
    fn default() -> Self {
        Self::Rust(RustConfiguration {})
    }
}

pub trait ConfigParser {
    fn parse(&self, path: PathBuf) -> anyhow::Result<Vec<Target>>;

    fn matches(&self, path: &Path) -> bool;
}

impl ConfigParser for LanguagesConfiguration {
    fn parse(&self, path: PathBuf) -> anyhow::Result<Vec<Target>> {
        match self {
            LanguagesConfiguration::CSharp(c) => c.parse(path),
            LanguagesConfiguration::Rust(r) => r.parse(path),
        }
    }

    fn matches(&self, path: &Path) -> bool {
        match &self {
            LanguagesConfiguration::Rust(r) => r.matches(path),
            LanguagesConfiguration::CSharp(c) => c.matches(path),
        }
    }
}

impl TryFrom<PathBuf> for LanguagesConfiguration {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        LanguagesConfiguration::iter()
            .find(|lang| lang.matches(&value))
            .ok_or(anyhow::anyhow!(
                "There is no implementation for that path yet"
            ))
    }
}
