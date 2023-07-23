mod cli;
mod cmd;
mod constants;
mod fs;
mod models;
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
    }
}
