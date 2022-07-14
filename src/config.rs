use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Config {
    root: String,
    items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    dir: String,
    root: PathBuf,
    files: Vec<String>,
    exclude: Vec<String>,
}

pub fn parse_config(path: &String) -> Config {
    println!("Checking config at {}", path);

    let content = read_to_string(Path::new(path))
        .expect(&format!("Unable to read config: {}", path));
    println!("{}", content);
    let yaml = YamlLoader::load_from_str(&content).unwrap();
    let config = yaml[0].as_hash().unwrap().iter().next().unwrap();
    let root = config.0.as_str().unwrap().to_string();
    println!("{:?}", root);
    let items: Vec<Item> = config.1.as_hash().unwrap().iter().map(|p| parse_item(p)).collect();
    println!("{:?}", items);
    Config { root, items }
}

fn parse_item(pair: (&Yaml, &Yaml)) -> Item {
    let dir = pair.0.as_str().unwrap().to_string();
    let root = PathBuf::from(pair.1["root"].as_str().unwrap());
    let files: Vec<String> = pair.1["files"].as_vec().unwrap().iter()
        .map(|it| it.as_str().unwrap().to_string()).collect();
    let exclude: Vec<String> = pair.1["exclude"].as_vec().unwrap().iter()
        .map(|it| it.as_str().unwrap().to_string()).collect();
    return Item { dir, root, files, exclude };
}
