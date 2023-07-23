mod cli;
mod cmd;
mod constants;
mod run;
mod utils;

use clap::Parser;
use cmd::{Opts, Subcommands};
use console::style;
use std::path::Path;
use std::{env, process};

use crate::{
    cli::utils::Cmd,
    constants::PALETTE_EMOJI,
    run::{run, Config},
    utils::crop_characters,
};

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    match opts.sub {
        Subcommands::Generate(cmd) => cmd.run(),
    };

    println!(
        "\n{} {}\n",
        style("We're generating some digital art!").yellow().bold(),
        PALETTE_EMOJI
    );

    Ok(())
    // let config = Config::new(env::args()).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    // if let Err(e) = run(Path::new(&config.dir)) {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }
    // // TODO: Error handling if asset folders don't follow required convention
    // // TODO: Error handling for .DS_STORE files
    // // TODO: Update piece of metadata without regenerating assets
    // // TODO: Wipe assets and regenerate everything
}
