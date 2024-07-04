use std::path::PathBuf;

use clap::Parser;

use crate::langs::LanguagesConfiguration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Output path for dot file
    #[clap(long)]
    pub output: PathBuf,
    #[clap(subcommand)]
    pub language: LanguagesConfiguration,
}
