use image::imageops::overlay;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::io::BufWriter;
use std::path::Path;
use std::{env, fs};

pub struct Config {
    pub dir: String,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<BTreeMap<String, String>>,
}

impl Config {
    // TODO Update this to use Clap instead of std::env::Args
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        };

        if args.len() > 2 {
            return Err("You should only pass the name of the folder housing the layer folders");
        };

        args.next();
        let dir = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a directory name"),
        };

        Ok(Config { dir })
    }
}

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    // TODO Abstract the collection size to a runtime argument
    let collection_size = 1000;

    // Create a HashMap to track which assets have been generated
    let mut asset_already_generated = HashMap::new();

    // TODO Make this a runtime argument
    let output_dir = "outputs";
    // Create an output directory to store the generated assets
    fs::create_dir_all(output_dir)?;

    let metadata_dir = "metadata";
    // Create a metadata directory to store the generated asset metadata
    fs::create_dir_all(metadata_dir)?;

    // TODO Put this into a dedicated function
    let mut rarity_tracker: Vec<Vec<(String, u32)>> = Vec::new();

    // Get the folders for each layer
    let mut subfolders: Vec<_> = fs::read_dir(path)?.map(|r| r.unwrap()).collect();

    // Sort the subfolders in alphanumeric order
    // The subfolder names should be prepended with a number corresponding to the desired order of layering
    // (e.g. 1<base_layer>, 2<middle_layer>, 3<top_layer>)
    subfolders.sort_by_key(|dir| dir.path());

    for folder in &subfolders {
        let mut layer_rarity: Vec<(String, u32)> = Vec::new();
        for i in fs::read_dir(folder.path()).unwrap() {
            let file = &i.unwrap().path();
            let rarity_weight: String = file
                .clone()
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .chars()
                .take(2)
                .collect();

            layer_rarity.push((
                file.display().to_string(),
                rarity_weight.parse::<u32>().unwrap(),
            ));
        }
        rarity_tracker.push(layer_rarity);
    }

    // Create the desired number of assets for the collection
    for i in 0..collection_size {
        // TODO: This should be updated to be specified in the config or as a CLI arg
        let (image_full_traits, base_layer_image, metadata) =
            gen_asset(path, &asset_already_generated, &rarity_tracker, i).unwrap();
        asset_already_generated.insert(image_full_traits, true);

        base_layer_image.save(format!("{}/{}.png", output_dir, i))?;

        let f = fs::File::create(format!("{}/{}.json", metadata_dir, i.to_string()))
            .expect("Unable to create the metadata file");
        let bw = BufWriter::new(f);
        serde_json::to_writer_pretty(bw, &metadata).expect("Unable to write the metadata file");

        println!("Generated ID {}", i);
    }

    Ok(())
}

pub fn gen_asset(
    path: &Path,
    asset_already_generated: &HashMap<BTreeSet<std::string::String>, bool>,
    rarity_tracker: &Vec<Vec<(String, u32)>>,
    i: u32,
) -> Result<
    (
        std::collections::BTreeSet<std::string::String>,
        image::DynamicImage,
        Metadata,
    ),
    Box<dyn Error>,
> {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    let base_dist = WeightedIndex::new(rarity_tracker[0].iter().map(|item| item.1)).unwrap();

    // Select the base layer
    let base_layer_selection = &rarity_tracker[0][base_dist.sample(&mut rng)].0;

    // Open the base layer image in order to be overlayed
    let mut base_layer_image = image::open(&base_layer_selection).unwrap();

    // Create a BTreeSet to store the top layers that get selected
    let mut top_layers = BTreeSet::new();

    let mut metadata_attributes: Vec<BTreeMap<String, String>> = Vec::new();

    for layer_rarity in &rarity_tracker[1..] {
        let layer_dist = WeightedIndex::new(layer_rarity.iter().map(|item| item.1)).unwrap();

        let file = &layer_rarity[layer_dist.sample(&mut rng)].0;
        top_layers.insert(file);
        let file_stem = Path::new(file)
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let parent_folder = Path::new(file)
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let cropped_layer_value = crop_characters(&file_stem, 2);

        let cropped_folder_name = crop_characters(&parent_folder, 2);
        let mut metadata_attribute_entries = BTreeMap::new();
        metadata_attribute_entries
            .insert("trait_type".to_string(), cropped_folder_name.to_string());
        metadata_attribute_entries.insert("value".to_string(), cropped_layer_value.to_string());

        metadata_attributes.push(metadata_attribute_entries);
    }

    // TODO Abstract metadata fields to a separate config
    let metadata = Metadata {
        name: format!("Asset #{}", i).to_owned(),
        description: "Description of the project".to_owned(),
        image: "https://project.mypinata.cloud/ipfs/hash/id.png".to_owned(),
        attributes: metadata_attributes,
    };

    // Create a BTreeSet to store all the layers
    let mut all_layers = BTreeSet::new();
    all_layers.insert(base_layer_selection.to_string());

    // Go through the toplayers and overlay the base layer with each toplayer in order
    for layer in top_layers {
        overlay(&mut base_layer_image, &image::open(&layer).unwrap(), 0, 0);
        all_layers.insert(layer.to_string());
    }

    // Check the final HashMap to see if this combination of layers has already been generated
    if asset_already_generated.contains_key(&all_layers) {
        // TODO Add operation in the case that no new assets can be generated
        // Recurse if the asset already exists
        gen_asset(path, asset_already_generated, rarity_tracker, i)?;
    }

    // TODO Create a struct for the asset to clean all of this up
    return Ok((all_layers, base_layer_image, metadata));
}

// TODO Move this to another file
// Utility function for cropping characters off of strings
fn crop_characters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
