use clap::Parser;
use std::path::PathBuf;
use std::{env, fs};

// Take in layers of art
#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: PathBuf,
}
// Overlay layers atop one another and check that assets are not duplicated

fn main() {
    println!("We're generating some digital art");
    let args = Cli::parse();
    let paths = fs::read_dir(&args.path).unwrap();

    for path in paths {
        let out = path.unwrap().path().display().to_string();
        println!("Name: {:?}", out.split("/").collect::<Vec<&str>>());
    }
    println!("{:?} and {:?}", args.pattern, args.path)
}

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
