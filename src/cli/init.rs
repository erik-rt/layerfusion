use clap::{Parser, ValueHint};
use std::path::PathBuf;

use crate::cli::utils::Cmd;

// TODO: Add struct fields and run logic

#[derive(Debug, Clone, Parser)]
pub struct InitArgs {
    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "PATH", default_value = ".")]
    root: PathBuf,
}

impl Cmd for InitArgs {
    fn run(self) -> eyre::Result<()> {
        todo!()
    }
}
