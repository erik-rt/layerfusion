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
use std::sync::Arc;

use crate::cli::utils::Cmd;
use crate::constants::{ASSETS_INPUT, ASSETS_OUTPUT, METADATA_OUTPUT, PALETTE_EMOJI};
use crate::fs::dir::Dir;
use crate::models::metadata::Metadata;

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

        let trait_layers = load_layers(input);

        // for s in subdirs {
        //     let dir = fs::read_dir(s)?;
        //
        //     for item in dir {
        //         println!("{:?}", item.unwrap().path().display())
        //     }
        // }

        // Create the assets output folder if it does not exist
        if !assets.exists() {
            // fs::create_dir_all(&assets)?;
        }

        // Create the metadata output folder if it does not exist
        if !metadata.exists() {
            // fs::create_dir_all(&metadata)?;
        }

        for i in 0..count {
            todo!()
        }

        Ok(())
    }
}

struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Debug)]
struct Layer {
    trait_type: String,
    trait_name: String,
    rarity: u32,
}

// impl Layer {
//     fn new(self, id: u32, trait_name: String, rarity: u32) -> Self {
//         Layer {
//             id,
//             trait_name,
//             rarity,
//         }
//     }
// }

fn create_artwork(layers: &[&Arc<Layer>]) {
    todo!()
}

fn encode_combination(layers: &[&Arc<Layer>]) -> eyre::Result<String> {
    todo!()
}

type ArtLayers = Vec<Vec<Box<Layer>>>;

fn load_layers(input_dir: PathBuf) -> eyre::Result<ArtLayers> {
    let subdirs = Dir::read_dir(input_dir)?.contents;

    // This flow actually might be better written imperatively so that error handling is easier
    let trait_layers = subdirs
        .into_iter()
        .map(|subdir| {
            let trait_type = subdir.file_stem().unwrap();

            // Logic for removing the layer order prefix. Still figuring out the order of
            // operations of this code.
            // If this is included, the `trait_type` Layer field needs to be updated to
            // `trait_type.to_owned().into_string().unwrap()`
            // let trait_type = trait_type.to_owned().into_string().unwrap();
            // let [_, trait_type]: [&str; 2] = trait_type
            //     .split("_")
            //     .collect::<Vec<&str>>()
            //     .try_into()
            //     .unwrap();

            let subdir = fs::read_dir(&subdir).unwrap();
            subdir
                .into_iter()
                .map(|file| {
                    // Get the file from within each subdirectory
                    let file = file.unwrap().file_name();
                    // Create a Path from the file
                    let file_path = Path::new(&file);
                    // Get the stem from the Path and convert it to a String
                    let file_stem = file_path
                        .file_stem()
                        .unwrap()
                        .to_owned()
                        .into_string()
                        .unwrap();

                    // Create a new Arc<Layer> and return
                    Box::new(Layer {
                        trait_type: trait_type.to_owned().into_string().unwrap(),
                        trait_name: file_stem,
                        rarity: 1,
                    })
                })
                .collect::<Vec<Box<Layer>>>()
        })
        .collect::<ArtLayers>();

    Ok(trait_layers)
}

fn generate_combinations(layers: &[&Arc<Layer>]) -> eyre::Result<()> {
    todo!()
}
