mod cli;

use clap::Parser;
use std::path::Path;
use std::{env, process};

use cli::Config;

// Take in layers of art
// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     #[clap(parse(from_os_str))]
//     path: PathBuf,
// }

fn main() {
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

// TODO: Overlay layers atop one another and check that assets are not duplicated

// TODO: Generate metadata associated with each asset, compliant with OS

// TODO: Update piece of metadata without regenerating assets

// TODO: Wipe assets and regenerate everything

// TODO: Add more assets to existing collection
