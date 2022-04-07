use image::imageops::overlay;
use rand::seq::IteratorRandom;
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

pub fn visit(path: &Path) -> Result<(), Box<dyn Error>> {
    for i in 0..10 {
        let mut bg = image::open("layers/backgrounds/lime.png").unwrap();
        let mut rng = rand::thread_rng();
        let mut overlays = Vec::<String>::with_capacity(fs::read_dir(path)?.count() - 1);
        for e in fs::read_dir(path)? {
            let e = e?;
            let path = e.path();
            println!("Dir: {:?}", path);
            println!("Folder: {:?}", path.file_stem().unwrap());

            // If the folder isn't the furthest background then push it to the array
            // TODO: Create a configuration file to set order of layers or create convention in file naming for ordering
            if path.file_stem().unwrap() != "backgrounds" {
                let files = fs::read_dir(path).unwrap();
                let file = files.choose(&mut rng).unwrap().unwrap();
                println!("File: {}", file.path().display());
                overlays.push(file.path().display().to_string());
            }
        }
        println!("{:#?}", overlays);
        for layer in overlays {
            overlay(&mut bg, &image::open(layer).unwrap(), 0, 0)
        }
        let output_dir = "outputs";
        fs::create_dir_all(output_dir)?;
        bg.save(format!("{}/output{}.png", output_dir, i))?;
    }

    Ok(())
}
