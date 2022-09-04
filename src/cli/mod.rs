use clap::{Parser, Subcommand};

use crate::constants::{ASSETS_INPUT, ASSETS_OUTPUT, CONFIG};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    // Log level: trace, debug, info, warn, error, off
    #[clap(short, long, global = true)]
    pub log_level: Option<String>,

    pub command: Commands,
}

pub enum Commands {
    // Interactive process to create the config file
    CreateConfig {
        // Path to the config file
        #[clap(short, long)]
        config: Option<String>,

        // Path to the directory with the assets
        #[clap(default_value = ASSETS_INPUT)]
        assets_input: String,
    },

    Generate {
        // Path to the config file
        #[clap(short, long)]
        config: Option<String>,

        // Path to the directory with the assets
        #[clap(default_value = ASSETS_INPUT)]
        assets_input: String,

        // Path to the directory with the assets
        #[clap(default_value = ASSETS_OUTPUT)]
        assets_output: String,
    },

    Hash {
        // Path to the directory with the assets
        #[clap(default_value = ASSETS_OUTPUT)]
        assets_output: String,
    },

    Update {
        // Path to the config file
        #[clap(short, long)]
        config: Option<String>,

        // Path to the directory with the assets
        #[clap(default_value = ASSETS_INPUT)]
        assets_input: String,

        // Path to the directory with the assets
        #[clap(default_value = ASSETS_OUTPUT)]
        assets_output: String,
    },
}
