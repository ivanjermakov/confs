use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use colored::Colorize;
use log::{debug, error, info, warn};

use crate::config::Config;
use crate::fs::{expand, matches};

// TODO: exclude support
pub fn check(config: &Config) {
    debug!("{:?}", config);
    config.items.iter().for_each(|item| {
        if Path::new(&expand(&item.root)).exists() {
            debug!("[{}] {} found", format!("{}", item.dir).green(), item.root);

            item.files.iter().for_each(|f| {
                let pattern = format!("{}{}", item.root, f);
                let files: Vec<PathBuf> = matches(&pattern);

                let matches = files.len();
                if matches == 0 {
                    warn!(
                        "[{}] {} -> no matches",
                        format!("{}", item.dir).green(),
                        pattern
                    )
                } else {
                    debug!(
                        "[{}] {} -> {} match{} [{:?}]",
                        format!("{}", item.dir).green(),
                        pattern,
                        matches,
                        if matches == 1 { "" } else { "es" },
                        files
                            .iter()
                            .map(|f| f.strip_prefix(expand(&item.root)).unwrap_or(f))
                            .collect::<Vec<&Path>>()
                    );
                }
            })
        } else {
            error!(
                "[{}] {} -> root not found",
                format!("{}", item.dir).green(),
                item.root
            )
        }
    });
    info!("Check completed");
}

// Fatal if:
// 1. config.[item].root is not present
pub fn check_fatal(config: &Config) -> Result<(), Error> {
    config
        .items
        .iter()
        .map(|item| {
            if Path::new(&expand(&item.root)).exists() {
                Ok(item)
            } else {
                Err(Error::new(
                    ErrorKind::NotFound,
                    format!("No such directory: {}", item.root),
                ))
            }
        })
        .map(|r| r.map(|_| ()))
        .collect()
}
