use clap::{Parser, ValueHint};
use console::style;
use image::{imageops, DynamicImage};
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
    /// Input directory
    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "INPUT PATH", default_value = ASSETS_INPUT)]
    input_dir: PathBuf,
    /// Output assets directory
    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "ASSETS OUTPUT PATH", default_value = ASSETS_OUTPUT)]
    assets_dir: PathBuf,
    /// Output metadata directory
    #[clap(short, long, value_hint = ValueHint::FilePath, value_name = "METADATA OUTPUT PATH", default_value = METADATA_OUTPUT)]
    metadata_dir: PathBuf,
}

impl Cmd for GenerateArgs {
    fn run(self) -> eyre::Result<()> {
        let GenerateArgs {
            count,
            input_dir,
            assets_dir,
            metadata_dir,
        } = self;

        println!(
            "\n{} {}",
            style("We're generating digital art!").yellow().bold(),
            PALETTE_EMOJI
        );

        if !input_dir.exists() {
            // TODO: Deal with these unwraps
            let mut cwd: String = env::current_dir()?.to_str().unwrap().to_owned();
            cwd.push_str("/");
            cwd.push_str(input_dir.to_str().unwrap());

            eyre::bail!("Directory {:?} does not exist", cwd)
        }

        let trait_layers = load_layers(input_dir)?;

        // Create the assets output folder if it does not exist
        if !assets_dir.exists() {
            fs::create_dir_all(&assets_dir)?;
        }

        // Create the metadata output folder if it does not exist
        if !metadata_dir.exists() {
            fs::create_dir_all(&metadata_dir)?;
        }

        let trait_layer_keys: Vec<String> = trait_layers.keys().cloned().collect();

        for i in 0..count {
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

            println!("Creating id {i}");
            let asset = create_artwork(&selected_layers)?;
            asset.save(format!("{}/{}.png", assets_dir.to_str().unwrap(), i))?;
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

type FinalImage = DynamicImage;

fn create_artwork(layers: &[&Box<Layer>]) -> eyre::Result<FinalImage> {
    // TODO: Add error handling rather than unwrap
    let canvas = &layers.first().unwrap();

    let mut canvas = image::open(&canvas.value)
        .map_err(|_| DirError::FileNotFoundError("Failed to open file".to_string()))?;

    // Skip the first element (the base layer)
    for layer in layers.iter().skip(1) {
        let layer = image::open(&layer.value)
            .map_err(|_| DirError::FileNotFoundError("Failed to open file".to_string()))?;
        imageops::overlay(&mut canvas, &layer, 0, 0)
    }

    Ok(canvas)
}

fn create_metadata(layers: &[&Box<Layer>]) -> eyre::Result<()> {
    todo!()
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
            let trait_value = file?.path();

            let rarity = 1;

            // Cloning since I need trait_type later as well
            let trait_type = trait_type.clone();

            let layer = Box::new(Layer::new(trait_type, trait_value, rarity));

            subdir_layers.push(layer);
        }

        trait_layers.insert(trait_type, subdir_layers);
    }

    Ok(trait_layers)
}

fn generate_combinations(layers: &[&Box<Layer>]) -> eyre::Result<()> {
    todo!()
}
