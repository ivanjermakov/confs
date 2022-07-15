extern crate glob;

use std::panic::set_hook;

use clap::{Parser, Subcommand};

mod check;
mod config;
mod fs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Check config validity")]
    Check(CheckArgs),
}

#[derive(Parser, Debug)]
struct CheckArgs {
    #[clap(short, long, value_parser, value_hint = clap::ValueHint::FilePath, default_value = "./confs.yml",
    help = "Path to config")]
    config_path: String,
    #[clap(short, long, required = false, takes_value = false, help = "Detailed output")]
    verbose: bool,
}

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    let cli = Cli::parse();
    match &cli.command {
        Commands::Check(args) => {
            let config = config::parse_config(&args.config_path);
            check::check(&config, args.verbose);
        }
    }
}
