use image::imageops::overlay;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
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
    for e in fs::read_dir(path)? {
        let e = e?;
        let path = e.path();
        if path.is_dir() {
            println!("Dir: {:?}", path);
            visit(&path)?;
        } else if path.is_file() {
            println!("File: {:?}", path);
            // Prints the name of the file (this is crucial for the metadata)
            println!("File Stem: {:?}", path.file_stem().unwrap());
            println!("Extension: {:?}", path.extension().unwrap());
            // let img = image::open(path).unwrap();
            // img.save("test.png").unwrap();
        }
    }
    Ok(())
}
