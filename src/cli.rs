use image::imageops::overlay;
use rand::seq::IteratorRandom;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::path::Path;
use std::{env, fs};

pub struct Config {
    pub dir: String,
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
    let collection_size = 50;

    // Create a HashMap to track which assets have been generated
    let mut asset_already_generated = HashMap::new();

    // TODO Make this a runtime argument
    let output_dir = "outputs";
    // Create an output directory to store the generated assets
    fs::create_dir_all(output_dir)?;

    // Create the desired number of assets for the collection
    for i in 0..collection_size {
        // TODO: This should be updated to be specified in the config or as a CLI arg
        let (image_full_traits, base_layer_image) =
            gen_asset(path, &asset_already_generated).unwrap();
        asset_already_generated.insert(image_full_traits, true);

        base_layer_image.save(format!("{}/{}.png", output_dir, i))?;
        println!("Generated asset {}", i);
    }

    Ok(())
}

pub fn gen_asset(
    path: &Path,
    asset_already_generated: &HashMap<BTreeSet<std::string::String>, bool>,
) -> Result<
    (
        std::collections::BTreeSet<std::string::String>,
        image::DynamicImage,
    ),
    Box<dyn Error>,
> {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Get the subfolders of the supplied folder. These subfolders should each correspond to a different asset layer
    let mut subfolders: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();

    // Sort the subfolders in alphanumeric order
    // The subfolder names should be prepended with a number corresponding to the desired order of layering
    // (e.g. 1<base_layer>, 2<middle_layer>, 3<top_layer>)
    subfolders.sort_by_key(|dir| dir.path());

    let base_layer_path = &subfolders[0].path();

    // Select the base layer
    let base_layer_selection = fs::read_dir(base_layer_path)
        .unwrap()
        .choose(&mut rng)
        .unwrap()
        .unwrap()
        .path();

    // Open the base layer image in order to be overlayed
    let mut base_layer_image = image::open(&base_layer_selection).unwrap();

    // Drop the base layer from the subfolders vector
    subfolders.drain(0..1);

    // Create a BTreeSet to store the top layers that get selected
    let mut top_layers = BTreeSet::new();

    // Loop through the toplayers and randomly select one from each folder
    for e in subfolders {
        let path = e.path();

        let files = fs::read_dir(path).unwrap();
        let file = files.choose(&mut rng).unwrap().unwrap();
        top_layers.insert(file.path().display().to_string());
    }

    // Create a BTreeSet to store all the layers
    let mut all_layers = BTreeSet::new();
    all_layers.insert(base_layer_selection.display().to_string());

    // Go through the toplayers and overlay the base layer with each toplayer in order
    for layer in top_layers {
        overlay(&mut base_layer_image, &image::open(&layer).unwrap(), 0, 0);
        all_layers.insert(layer);
    }

    // Check the final HashMap to see if this combination of layers has already been generated
    if asset_already_generated.contains_key(&all_layers) {
        // TODO Add operation in the case that no new assets can be generated
        // Recurse if the asset already exists
        gen_asset(path, asset_already_generated)?;
    }
    // TODO Create a struct for the asset to clean all of this up
    return Ok((all_layers, base_layer_image));
}

// TODO Move this to another file
// Utility function for cropping characters off of strings
fn crop_characters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
