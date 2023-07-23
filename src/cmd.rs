use crate::cli::generate::GenerateArgs;
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
    /// Generate assets
    Generate(GenerateArgs),
}
