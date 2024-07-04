use clap::Subcommand;
use rust::RustConfiguration;

use crate::model::Target;

mod rust;

#[derive(Subcommand, Debug)]
pub enum LanguagesConfiguration {
    Rust(RustConfiguration),
}

pub trait ConfigParser {
    fn parse(&self) -> anyhow::Result<Vec<Target>>;
}
