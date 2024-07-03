use clap::Parser;

use crate::langs::LanguagesConfiguration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub language: LanguagesConfiguration,
}
