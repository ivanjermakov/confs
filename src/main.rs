extern crate glob;

use std::panic::set_hook;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use log::error;
use log::LevelFilter::{Info, Trace};

use crate::command::check::check;
use crate::command::sync::sync;

mod command;
mod config;
mod fs;
mod logger;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(
        global = true,
        short,
        long,
        value_parser,
        value_hint = clap::ValueHint::FilePath,
        default_value = "./confs.yml",
        help = "Path to config"
    )]
    config_path: String,

    #[clap(
        global = true,
        short,
        long,
        required = false,
        takes_value = false,
        help = "Detailed output"
    )]
    verbose: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Check config validity")]
    Check,
    #[clap(about = "Sync files into local directory according to config")]
    Sync,
}

fn main() -> Result<()> {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            error!("{}", s);
        }
    }));

    let cli = Cli::parse();
    logger::init(if cli.verbose { Trace } else { Info }).map_err(|e| anyhow!(e))?;
    let config = config::parse_config(&cli.config_path)?;
    match &cli.command {
        Commands::Check => check(&config),
        Commands::Sync => sync(&config),
    };

    Ok(())
}
