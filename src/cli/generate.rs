use clap::{Parser, ValueHint};
use console::style;
use image::imageops::overlay;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::cli::utils::Cmd;
use crate::constants::{ASSETS_INPUT, ASSETS_OUTPUT, METADATA_OUTPUT, PALETTE_EMOJI};
use crate::fs::dir::Dir;
use crate::models::metadata::Metadata;
use crate::utils::crop_characters;

#[derive(Debug, Clone, Parser)]
pub struct GenerateArgs {
    #[clap(short, long)]
    count: u128,

    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "INPUT PATH", default_value = ASSETS_INPUT)]
    input: PathBuf,

    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "ASSETS OUTPUT PATH", default_value = ASSETS_OUTPUT)]
    assets: PathBuf,

    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "METADATA OUTPUT PATH", default_value = METADATA_OUTPUT)]
    metadata: PathBuf,
}

impl Cmd for GenerateArgs {
    fn run(self) -> eyre::Result<()> {
        let GenerateArgs {
            count,
            input,
            assets,
            metadata,
        } = self;

        println!(
            "\n{} {}",
            style("We're generating digital art!").yellow().bold(),
            PALETTE_EMOJI
        );

        if !input.exists() {
            let mut cwd: String = env::current_dir()?.to_str().unwrap().to_owned();
            cwd.push_str("/");
            cwd.push_str(input.to_str().unwrap());

            eyre::bail!("Directory {:?} does not exist", cwd)
        }

        let input = Dir::read_dir(input)?;
        let contents = input.contents;

        // TODO: Use iterator instead of imperative syntax
        for c in contents {
            let dir = fs::read_dir(c).unwrap();
            for item in dir {
                println!("{:?}", item.unwrap().path().display())
            }
        }
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
