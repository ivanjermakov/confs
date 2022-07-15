use std::path::PathBuf;

use glob::glob;
use shellexpand::tilde;

pub fn matches(pattern: &String) -> Vec<PathBuf> {
    return glob(&tilde(&pattern))
        .expect(&format!("Invalid pattern {}", pattern))
        .map(|it| it.unwrap())
        .collect();
}
