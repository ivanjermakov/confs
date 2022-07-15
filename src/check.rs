use std::path::PathBuf;

use crate::config::Config;
use crate::fs::matches;

// TODO: exclude support
pub fn check(config: &Config, verbose: bool) {
    println!("{:?}", config);
    config.items.iter().for_each(|item| {
        item.files.iter().for_each(|f| {
            let pattern = format!("{}{}", item.root, f);
            let files: Vec<PathBuf> = matches(&pattern);

            let matches = files.len();
            if matches == 0 {
                println!("Pattern {}: no matches", pattern)
            } else if verbose {
                println!(
                    "Pattern {}: {} match{}",
                    pattern,
                    matches,
                    if matches == 1 { "" } else { "es" }
                );
            }
        })
    });
    println!("Check completed");
}
