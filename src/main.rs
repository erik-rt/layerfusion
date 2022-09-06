// use anyhow::Result;
use console::style;
use std::error::Error;
use std::path::Path;
use std::{env, process};

use artengine_rs::{
    constants::{CHECKMARK_EMOJI, ERROR_EMOJI, PALETTE_EMOJI},
    logic::{logic, Config},
    utils::*,
};

fn main() {
    println!(
        "\n{}{}",
        PALETTE_EMOJI,
        style("Time to overhaul the codebase!").cyan().bold()
    );

    match run() {
        Ok(()) => {
            println!(
                "\n{}{}",
                CHECKMARK_EMOJI,
                style("Application successful!").green().bold()
            );
        }
        Err(err) => {
            eprintln!(
                "\n{}{} {}",
                ERROR_EMOJI,
                style("Application error:").red(),
                err,
            );
            // finished the program with an error code to the OS
            process::exit(1);
        }
    }
    // println!(
    //     "\n{} {}\n",
    //     style("We're generating some digital art!").yellow().bold(),
    //     PALETTE_EMOJI
    // );

    // TODO: Error handling if asset folders don't follow required convention
    // TODO: Error handling for .DS_STORE files
    // TODO: Update piece of metadata without regenerating assets
    // TODO: Wipe assets and regenerate everything
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = logic(Path::new(&config.dir)) {
        return Err(err);
    }
    Ok(())
}
