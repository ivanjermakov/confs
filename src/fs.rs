use std::path::PathBuf;

use glob::glob;
use log::debug;
use shellexpand::tilde;

use crate::config::Item;

pub fn expand(path: &str) -> String {
    tilde(path).to_string()
}

pub fn matches(pattern: &str) -> Vec<PathBuf> {
    let pattern = expand(pattern);
    let res = glob(&pattern)
        .expect(&format!("Invalid pattern {}", pattern))
        .map(|it| it.unwrap())
        .collect();
    debug!("Glob {pattern}: {res:?}");
    res
}

pub fn item_matches(item: &Item) -> Vec<PathBuf> {
    let excluded_matches = item_excluded_files(item);
    item.files
        .iter()
        .map(|f| item.join(f))
        .flat_map(|p| matches(&p))
        .filter(|p| !excluded_matches.iter().any(|e| e.eq(p)))
        .collect()
}

pub fn item_excluded_files(item: &Item) -> Vec<PathBuf> {
    item.exclude
        .iter()
        .map(|f| item.join(f))
        .flat_map(|p| matches(&p))
        .collect()
}
