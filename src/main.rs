mod cli;

use clap::Parser;
use image::imageops::overlay;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
// use failure::Error;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, process};

use cli::Config;

// Take in layers of art
// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     #[clap(parse(from_os_str))]
//     path: PathBuf,
// }

fn main() {
    let mut bg = image::open("layers/backgrounds/lime.png").unwrap();
    let eyes = image::open("layers/eyes/horizontal_copy.png").unwrap();

    // println!("dimensions: {:?}", img1);
    // println!("color: {:?}", img1.pixels());
    let face = image::open("layers/faces/face_copy.png").unwrap();
    let bod = image::open("layers/bodies/blue.png").unwrap();
    let hat = image::open("layers/hats/red.png").unwrap();

    // let F: Vec<u16> = img1.iter().zip(face.iter()).map(|(&b, &v)| b + v).collect();
    overlay(&mut bg, &face, 0, 0);
    overlay(&mut bg, &eyes, 0, 0);
    overlay(&mut bg, &bod, 0, 0);
    overlay(&mut bg, &hat, 0, 0);
    bg.save("output.png").unwrap();

    println!("We're generating some digital art");
    // let args = Cli::parse();
    // let paths = fs::read_dir(args.path).unwrap();

    // TODO: Error handling if asset folders don't follow required convention

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = cli::visit(Path::new(&config.dir)) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// Overlay layers atop one another and check that assets are not duplicated
fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

fn generate_asset() {
    println!("{:?}", env::current_dir())
}

// Generate metadata associated with each asset, compliant with OS

// Update piece of metadata without regenerating assets

// Wipe assets and regenerate everything

// Add more assets to existing collection
