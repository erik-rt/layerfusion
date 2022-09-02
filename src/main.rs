use console::style;
use std::path::Path;
use std::{env, process};

use art_engine_rs::{cli::*, constants::PALETTE_EMOJI, utils::*};

fn main() {
    println!(
        "\n{} {}\n",
        style("Time to overhaul the codebase!").cyan().bold(),
        PALETTE_EMOJI
    );
    // println!("We're generating some digital art");

    // let config = Config::new(env::args()).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    // if let Err(e) = cli::run(Path::new(&config.dir)) {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }
    // // TODO: Error handling if asset folders don't follow required convention
    // // TODO: Error handling for .DS_STORE files
    // // TODO: Update piece of metadata without regenerating assets
    // // TODO: Wipe assets and regenerate everything
}
