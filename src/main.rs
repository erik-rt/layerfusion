use clap::Parser;
// use failure::Error;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, process};

// Take in layers of art
#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

struct Config {
    dir: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let dir = args[1].clone();

        Ok(Config { dir })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("We're generating some digital art");
    // let args = Cli::parse();
    // let paths = fs::read_dir(args.path).unwrap();

    // TODO: Write code that accepts folder instead of hardcoding a folder name
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    visit(Path::new(&config.dir))
}

fn visit(path: &Path) -> Result<(), Box<dyn Error>> {
    for e in fs::read_dir(path)? {
        let e = e?;
        let path = e.path();
        if path.is_dir() {
            println!("Dir: {:?}", path);
            visit(&path)?;
        } else if path.is_file() {
            println!("File: {:?}", path);
        }
    }
    Ok(())
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
