use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct RustConfiguration {
    pub path: PathBuf,
}
