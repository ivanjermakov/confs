use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use log::{debug, error, info, warn};

use crate::config::{pretty_item, Config};
use crate::fs::{expand, matches};

// TODO: exclude support
pub fn check(config: &Config) {
    debug!("{:?}", config);
    config.items.iter().for_each(|item| {
        if Path::new(&expand(&item.root)).exists() {
            debug!("{} {} found", pretty_item(item), item.root);

            item.files.iter().for_each(|f| {
                let pattern = item.join(f);
                let files: Vec<PathBuf> = matches(&pattern);

                let matches = files.len();
                if matches == 0 {
                    warn!("{} {} -> no matches", pretty_item(item), pattern)
                } else {
                    debug!(
                        "{} {} -> {} match{} [{:?}]",
                        pretty_item(item),
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
            error!("{} {} -> root not found", pretty_item(item), item.root)
        }
    });
    info!("Check completed");
}

// Fatal if:
// 1. config.[item].root is not present
pub fn check_fatal(config: &Config) -> Result<(), Error> {
    config.items.iter().try_for_each(|item| {
        if Path::new(&expand(&item.root)).exists() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("No such directory: {}", item.root),
            ))
        }
    })
}
