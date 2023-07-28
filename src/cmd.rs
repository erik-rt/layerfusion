use crate::cli::generate::GenerateArgs;
use crate::cli::init::InitArgs;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "Art Engine TM")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(about = "Generate your latest profile picture.")]
pub enum Subcommands {
    /// Initialize new project
    Init(InitArgs),
    /// Generate assets
    Generate(GenerateArgs),
}
