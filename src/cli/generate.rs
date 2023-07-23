use crate::cli::utils::Cmd;
use clap::{Parser, ValueHint};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Parser)]
pub struct GenerateArgs {
    #[clap(short, long)]
    count: u128,

    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "ASSETS OUTPUT PATH", default_value = "assets")]
    assets: PathBuf,

    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "METADATA OUTPUT PATH", default_value = "metadata")]
    metadata: PathBuf,
}

impl Cmd for GenerateArgs {
    fn run(self) -> eyre::Result<()> {
        let GenerateArgs {
            count,
            assets,
            metadata,
        } = self;

        // Create the assets output folder if it does not exist
        if !assets.exists() {
            // fs::create_dir_all(&assets)?;
        }

        // Create the metadata output folder if it does not exist
        if !metadata.exists() {
            // fs::create_dir_all(&metadata)?;
        }
        Ok(())
    }
}
