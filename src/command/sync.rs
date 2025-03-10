use std::fs::{copy, create_dir_all};
use std::path::Path;

use log::{debug, info};

use crate::command::check::check_fatal;
use crate::config::{pretty_item, Config};
use crate::fs::{expand, item_matches};

pub fn sync(config: &Config) {
    let root = expand(&config.root);
    let root_path = Path::new(&root);
    let mut copy_count = 0;

    check_fatal(config).and_then(|_| create_dir_all(root_path)).unwrap();

    config.items.iter().for_each(|item| {
        let matches = item_matches(item);
        let dest = root_path.join(&item.dir);
        create_dir_all(&dest).unwrap();
        matches.iter().for_each(|f| {
            debug!("{} Comparing files: {:?} | {:?}", pretty_item(item), f, &dest);
            if dest.exists() {
                debug!("\tDestination file exists");
            }
            // TODO: strip prefix
            info!("{} Copying file: {:?} -> {:?}", pretty_item(item), f, &dest);
            copy(f, &dest).unwrap();
            copy_count += 1;
        })
    });
    info!("Copied {} files across {} items", copy_count, config.items.len())
}
