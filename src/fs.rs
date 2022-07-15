use std::path::{Path, PathBuf};

use glob::glob;
use shellexpand::tilde;

use crate::config::Item;

pub fn expand(path: &String) -> String {
    let err = format!("Error expanding to full path: {}", path);
    Path::new(&tilde(path).to_string())
        .to_str()
        .expect(&err)
        .to_string()
}

pub fn matches(pattern: &String) -> Vec<PathBuf> {
    return glob(&expand(pattern))
        .expect(&format!("Invalid pattern {}", pattern))
        .map(|it| it.unwrap())
        .collect();
}

pub fn item_matches(item: &Item) -> Vec<PathBuf> {
    let excluded_matches = item_excluded_files(item);
    item.files
        .iter()
        .map(|f| format!("{}{}", item.root, f))
        .flat_map(|p| matches(&p))
        .filter(|p| !excluded_matches.iter().any(|e| e.eq(p)))
        .collect()
}

pub fn item_excluded_files(item: &Item) -> Vec<PathBuf> {
    item.exclude
        .iter()
        .map(|f| format!("{}{}", item.root, f))
        .flat_map(|p| matches(&p))
        .collect()
}
