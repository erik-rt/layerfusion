mod cli;
mod cmd;
mod config;
mod constants;
mod fs;
mod models;

use clap::Parser;
use cmd::{Opts, Subcommands};

use crate::cli::utils::Cmd;

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    match opts.sub {
        Subcommands::Generate(cmd) => cmd.run(),
    }
}
