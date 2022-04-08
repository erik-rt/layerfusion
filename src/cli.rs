use image::imageops::overlay;
use rand::seq::IteratorRandom;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::path::Path;
use std::{env, fs};

pub struct Config {
    pub dir: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        };

        args.next();
        let dir = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a directory name"),
        };

        Ok(Config { dir })
    }
}

pub fn gen_asset(
    path: &Path,
    final_assets: &HashMap<BTreeSet<std::string::String>, bool>,
) -> Result<
    (
        std::collections::BTreeSet<std::string::String>,
        image::DynamicImage,
    ),
    Box<dyn Error>,
> {
    let mut rng = rand::thread_rng();
    let base_trait_name = "backgrounds";
    let path_to_base_trait = Path::new(path).join(base_trait_name);

    let base_trait_selection = fs::read_dir(path_to_base_trait)
        .unwrap()
        .choose(&mut rng)
        .unwrap()
        .unwrap()
        .path();

    let mut base_trait = image::open(&base_trait_selection).unwrap();

    let mut layer_traits = HashSet::new();
    for e in fs::read_dir(path)? {
        let e = e?;
        let path = e.path();

        // TODO: Create a configuration file to set order of layer_traits or create convention in file naming for ordering
        if path.file_stem().unwrap() != "backgrounds" {
            let files = fs::read_dir(path).unwrap();
            let file = files.choose(&mut rng).unwrap().unwrap();
            layer_traits.insert(file.path().display().to_string());
        }
    }

    let mut asset_all_traits = BTreeSet::new();
    asset_all_traits.insert(base_trait_selection.display().to_string());
    for layer in layer_traits {
        overlay(&mut base_trait, &image::open(&layer).unwrap(), 0, 0);
        asset_all_traits.insert(layer);
    }
    // TODO Add operation in the case that no new assets can be generated
    if final_assets.contains_key(&asset_all_traits) {
        // Recurse if the asset metadata already exists
        gen_asset(path, final_assets)?;
    }
    // TODO Create a struct for the asset to clean all of this up
    return Ok((asset_all_traits, base_trait));
}

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    // TODO Abstract the collection size to a runtime argument
    let collection_size = 50;
    let mut asset_already_generated = HashMap::new();
    for i in 0..collection_size {
        // TODO: This should be updated to be specified in the config or as a CLI arg
        let (image_full_traits, base_trait) = gen_asset(path, &asset_already_generated).unwrap();
        asset_already_generated.insert(image_full_traits, true);

        let output_dir = "outputs";
        fs::create_dir_all(output_dir)?;
        base_trait.save(format!("{}/output{}.png", output_dir, i))?;
    }

    Ok(())
}
