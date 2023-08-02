use clap::{Parser, ValueHint};
use console::style;
use image::{imageops, DynamicImage};
use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::BufWriter;
use std::path::PathBuf;

use crate::cli::utils::Cmd;
use crate::cli::ConversionError;
use crate::constants::{ASSETS_INPUT, ASSETS_OUTPUT, METADATA_OUTPUT, PALETTE_EMOJI};
use crate::fs::dir::{Dir, DirError};
use crate::models::metadata::{Attribute, Metadata};

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

        let num_generated = fs::read_dir(&assets_dir)
            .map_err(|_| DirError::DirectoryNotFoundError("Could not find directory".to_string()))?
            .count();

        let mut used_combinations: HashSet<String> = HashSet::new();

        for i in 0..count {
            let current_id = (i as usize) + num_generated;

            let mut selected_layers: Vec<&Box<Layer>> = Vec::new();

            for key in &trait_layer_keys {
                let mut rng = thread_rng();

                let layer = match trait_layers.get(key) {
                    Some(l) => l.choose_weighted(&mut rng, |x| x.rarity).unwrap(),
                    // TODO: Return a descriptive error rather than bailing
                    None => eyre::bail!("Could not find layers for trait type"),
                };
                selected_layers.push(layer);
            }

            let encoded_combination = encode_combination(&selected_layers)?;

            // TODO: This currently will continue the count if a duplicate is found
            // This needs to be updated so that the count will continue where it left off
            if !used_combinations.contains(&encoded_combination) {
                let asset = create_artwork(&selected_layers)?;
                let metadata = create_metadata(&selected_layers, current_id)?;

                // Save image to assets_dir
                asset.save(format!(
                    "{}/{}.png",
                    assets_dir.to_str().unwrap(),
                    current_id
                ))?;

                // Write Metadata file to metadata_dir
                let f = fs::File::create(format!(
                    "{}/{}",
                    metadata_dir.to_str().unwrap(),
                    current_id.to_string()
                ))?;
                let bw = BufWriter::new(f);
                serde_json::to_writer_pretty(bw, &metadata)?;

                // Insert trait_value combination encoding
                used_combinations.insert(encoded_combination);

                println!(
                    "{}",
                    style(format!("Generated ID {current_id}")).cyan().bold()
                );
            }
        }
        Ok(())
    }
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

const PROJECT_DESCRIPTION: &'static str = "Generic project description";

fn create_metadata(layers: &[&Box<Layer>], current_id: usize) -> eyre::Result<Metadata> {
    let name = String::from(format!("Generic Project #{current_id}"));
    let description = String::from(PROJECT_DESCRIPTION);
    let image = String::from(format!("ar://hash/{current_id}.png"));

    let mut attributes: Vec<Attribute> = Vec::new();

    for layer in layers {
        let trait_type = &layer.trait_type;
        // TODO: Add error handling
        let trait_type = trait_type.split("_").last().unwrap().to_string();

        // TODO: Add error handling
        let value = layer.value.file_stem().unwrap();
        let value = value.to_str().unwrap().to_string();

        let attr = Attribute {
            trait_type: trait_type.to_string(),
            value,
        };

        attributes.push(attr);
    }

    let metadata = Metadata::new(name, description, image, attributes);

    Ok(metadata)
}

fn encode_combination(layers: &[&Box<Layer>]) -> eyre::Result<String> {
    let encoding = layers
        .iter()
        // TODO: Add error handling
        .map(|layer| layer.value.file_stem().unwrap().to_str().unwrap())
        .collect::<Vec<&str>>()
        .join("-");

    Ok(encoding)
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
