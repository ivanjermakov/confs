use std::panic::set_hook;

use clap::{Parser, Subcommand, ValueHint};

mod check;
mod config;

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
    #[clap(short, long, value_parser, value_hint = ValueHint::FilePath, default_value = "./confs.yml")]
    config_path: String,
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
            check::check(&config);
        }
    }
}
