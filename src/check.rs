use std::path::{Path, PathBuf};

use colored::Colorize;
use log::{debug, error, info, warn};
use shellexpand::tilde;

use crate::config::Config;
use crate::fs::matches;

// TODO: exclude support
pub fn check(config: &Config) {
    debug!("{:?}", config);
    config.items.iter().for_each(|item| {
        if Path::new(&tilde(&item.root).to_string()).exists() {
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
                            .map(|f| f.strip_prefix(&tilde(&item.root).to_string()).unwrap_or(f))
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
