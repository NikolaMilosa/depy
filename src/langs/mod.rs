use clap::Subcommand;
use csharp::CSharpConfiguration;
use rust::RustConfiguration;

use crate::model::Target;

mod csharp;
mod rust;

#[derive(Subcommand, Debug)]
pub enum LanguagesConfiguration {
    Rust(RustConfiguration),
    CSharp(CSharpConfiguration),
}

pub trait ConfigParser {
    fn parse(&self) -> anyhow::Result<Vec<Target>>;
}

impl ConfigParser for LanguagesConfiguration {
    fn parse(&self) -> anyhow::Result<Vec<Target>> {
        match self {
            LanguagesConfiguration::CSharp(c) => c.parse(),
            LanguagesConfiguration::Rust(r) => r.parse(),
        }
    }
}
