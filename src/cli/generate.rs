use clap::{Parser, ValueHint};
use console::style;
use image::imageops::overlay;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::cli::utils::Cmd;
use crate::cli::ConversionError;
use crate::constants::{ASSETS_INPUT, ASSETS_OUTPUT, METADATA_OUTPUT, PALETTE_EMOJI};
use crate::fs::dir::{Dir, DirError};
use crate::models::metadata::Metadata;

// TODO: Abstract a lot of the types away to structs and types

#[derive(Debug, Clone, Parser)]
pub struct GenerateArgs {
    /// Number of assets to generate
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

        let trait_layers = load_layers(input)?;

        // Create the assets output folder if it does not exist
        if !assets.exists() {
            // fs::create_dir_all(&assets)?;
        }

        // Create the metadata output folder if it does not exist
        if !metadata.exists() {
            // fs::create_dir_all(&metadata)?;
        }

        let trait_layer_keys: Vec<String> = trait_layers.keys().cloned().collect();
        println!("{:?}", trait_layer_keys);

        for _ in 0..count {
            let selected_layers: Vec<&Box<Layer>> = trait_layer_keys
                .iter()
                .map(|trait_type| {
                    let mut rng = thread_rng();

                    let layer = match trait_layers.get(trait_type) {
                        Some(l) => l.choose_weighted(&mut rng, |x| x.rarity).unwrap(),
                        // TODO: Add a more descriptive error
                        None => eyre::bail!("Could not find layers for trait type"),
                    };

                    Ok(layer)
                })
                .map(|l| l.unwrap())
                .collect();

            println!("{:?}", selected_layers);

            let asset = create_artwork(&selected_layers);
        }
        todo!()
    }
}

struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Debug)]
struct Layer {
    /// Trait type of the layer (e.g., background, foreground, body, etc.)
    trait_type: String,
    /// Value of the relative trait type
    value: PathBuf,
    /// Probability of being selected relative to other layers
    rarity: u32,
}

impl Layer {
    fn new(trait_type: String, value: PathBuf, rarity: u32) -> Self {
        Layer {
            trait_type,
            value,
            rarity,
        }
    }
}

fn create_artwork(layers: &[&Box<Layer>]) {
    // TODO: Add error handling rather than unwrap
    let base_layer = &layers.first().unwrap();
}

fn encode_combination(layers: &[&Box<Layer>]) -> eyre::Result<String> {
    todo!()
}

type TraitLayers = BTreeMap<String, Vec<Box<Layer>>>;

fn load_layers(input_dir: PathBuf) -> eyre::Result<TraitLayers> {
    let subdirs = Dir::read_dir(input_dir)?.contents;

    let mut trait_layers: TraitLayers = BTreeMap::new();

    for subdir in subdirs {
        let trait_type = subdir
            .file_stem()
            .ok_or(DirError::FileStemError(
                "Error reading file stem".to_string(),
            ))?
            .to_owned()
            .into_string()
            .map_err(|_| {
                // TODO: Update the following error to be more descriptive
                ConversionError::OsStringToStringError(
                    "Failed to convert OsString to String".to_string(),
                )
            })?;

        let subdir = fs::read_dir(&subdir)?;

        let mut subdir_layers: Vec<Box<Layer>> = vec![];

        for file in subdir {
            let file = file?.file_name();

            let file_path = Path::new(&file);

            let trait_value = file_path.file_stem().ok_or(DirError::FileStemError(
                "Error reading file stem.".to_string(),
            ))?;

            let rarity = 1;

            // Cloning since I need trait_type later as well
            let trait_type = trait_type.clone();

            let layer = Box::new(Layer::new(trait_type, trait_value.into(), rarity));

            subdir_layers.push(layer);
        }

        trait_layers.insert(trait_type, subdir_layers);
    }

    Ok(trait_layers)
}

fn generate_combinations(layers: &[&Box<Layer>]) -> eyre::Result<()> {
    todo!()
}
