use colored::{ColoredString, Colorize};
use std::fs::read_to_string;
use std::path::Path;

use log::debug;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Config {
    pub root: String,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    pub root: String,
    pub name: String,
    pub files: Vec<String>,
    pub exclude: Vec<String>,
}

impl Item {
    pub fn join(&self, file_glob: &str) -> String {
        format!("{}{}", self.root, file_glob)
    }
}

pub fn parse_config(path: &String) -> Config {
    debug!("Config check {}", path);
    let generic_error = "Config error";

    let content = read_to_string(Path::new(path)).expect(&format!("Unable to read config: {}", path));

    let yaml = YamlLoader::load_from_str(&content).expect(generic_error);

    let config = yaml[0].as_hash().and_then(|it| it.iter().next()).expect(generic_error);

    let root = config.0.as_str().expect(generic_error).to_string();

    let items: Vec<Item> = config
        .1
        .as_hash()
        .expect(generic_error)
        .iter()
        .map(|p| parse_item(p))
        .collect();

    Config { root, items }
}

fn parse_item(pair: (&Yaml, &Yaml)) -> Item {
    let generic_error = "Config error: Error parsing item";

    let name = pair.0.as_str().expect(generic_error).to_string();

    let item_error = &format!("{} {}", generic_error, name);

    let root = pair.1["root"].as_str().expect(item_error).to_string();

    let files: Vec<String> = pair.1["files"]
        .as_vec()
        .expect(item_error)
        .iter()
        .map(|it| it.as_str().expect(item_error).to_string())
        .collect();

    let exclude: Vec<String> = pair.1["exclude"].as_vec().map_or(vec![], |it| {
        it.iter().map(|it| it.as_str().expect(item_error).to_string()).collect()
    });

    Item {
        name,
        root,
        files,
        exclude,
    }
}

pub fn pretty_item(item: &Item) -> ColoredString {
    format!("[{}]", format!("{}", item.name).green()).normal()
}
