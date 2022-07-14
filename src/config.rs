use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    root: String,
    items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    root: PathBuf,
    files: Vec<String>,
    exclude: Vec<String>,
}

pub fn parse_config(path: &String) -> Option<Config> {
    let full_path = get_full_path(path);
    println!("Checking config at {}", full_path.unwrap().display());

    todo!()
}

fn get_full_path(path: &String) -> Option<PathBuf> {
    let full_path = Path::new(path).canonicalize();
    if full_path.is_err() {
        println!("Path to config {} does not exist", path);
        return None;
    }
    return Some(full_path.unwrap());
}
