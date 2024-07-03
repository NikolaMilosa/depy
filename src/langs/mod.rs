use clap::Subcommand;
use rust::RustConfiguration;

mod rust;

#[derive(Subcommand, Debug)]
pub enum LanguagesConfiguration {
    Rust(RustConfiguration),
}
