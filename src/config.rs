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
    let generic_error = "Config error";

    let content = read_to_string(Path::new(path))
        .expect(&format!("Unable to read config: {}", path));

    let yaml = YamlLoader::load_from_str(&content)
        .expect(generic_error);

    let config = yaml[0].as_hash()
        .and_then(|it| it.iter().next())
        .expect(generic_error);

    let root = config.0.as_str()
        .expect(generic_error)
        .to_string();

    let items: Vec<Item> = config.1.as_hash()
        .expect(generic_error).iter()
        .map(|p| parse_item(p)).collect();

    Config { root, items }
}

fn parse_item(pair: (&Yaml, &Yaml)) -> Item {
    let generic_error = "Config error: Error parsing item";

    let dir = pair.0.as_str()
        .expect(generic_error).to_string();

    let item_error = &format!("{} {}", generic_error, dir);

    let root = PathBuf::from(pair.1["root"].as_str()
        .expect(item_error));

    let files: Vec<String> = pair.1["files"].as_vec()
        .expect(item_error).iter()
        .map(|it| it.as_str()
            .expect(item_error).to_string()
        ).collect();

    let exclude: Vec<String> = pair.1["exclude"].as_vec()
        .expect(item_error).iter()
        .map(|it| it.as_str()
            .expect(item_error).to_string())
        .collect();

    return Item { dir, root, files, exclude };
}
