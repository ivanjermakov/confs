use anyhow::{Context, Error, Result};
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

pub fn parse_config(path: &String) -> Result<Config> {
    debug!("Config check {}", path);
    let content = read_to_string(Path::new(path)).context(format!("Read error: {}", path))?;
    let yaml = YamlLoader::load_from_str(&content).context("Parse config")?;
    let config = yaml[0]
        .as_hash()
        .and_then(|it| it.iter().next())
        .context("Parse config")?;
    let root = config.0.as_str().context("Parse root")?.to_string();
    let items: Vec<Item> = config
        .1
        .as_hash()
        .context("Parse items")?
        .iter()
        .map(parse_item)
        .collect::<Result<_, _>>()?;
    Ok(Config { root, items })
}

fn parse_item(pair: (&Yaml, &Yaml)) -> Result<Item> {
    let name = pair.0.as_str().context("Parse item")?.to_string();
    let root = pair.1["root"].as_str().context("Parse item")?.to_string();

    let files: Vec<String> = pair.1["files"]
        .as_vec()
        .context("Parse files")?
        .iter()
        .map(|it| Ok(it.as_str().context("Parse file")?.to_string()))
        .collect::<Result<Vec<_>, Error>>()?;

    let exclude: Vec<String> = pair.1["exclude"].as_vec().map_or(Ok(vec![]), |it| {
        it.iter()
            .map(|it| Ok(it.as_str().context("Parse exclude item")?.to_string()))
            .collect::<Result<Vec<_>, Error>>()
    })?;

    Ok(Item {
        name,
        root,
        files,
        exclude,
    })
}

pub fn pretty_item(item: &Item) -> ColoredString {
    format!("[{}]", format!("{}", item.name).green()).normal()
}
