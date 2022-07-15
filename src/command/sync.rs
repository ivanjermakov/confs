use std::fs::create_dir_all;
use std::path::Path;

use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use log::{debug, info};

use crate::command::check::check_fatal;
use crate::config::{pretty_item, Config};
use crate::fs::{expand, item_matches};

pub fn sync(config: &Config) {
    let root = expand(&config.root);
    let root_path = Path::new(&root);
    let mut copy_count = 0;

    check_fatal(config)
        .and_then(|_| create_dir_all(root_path))
        .unwrap();

    config.items.iter().for_each(|item| {
        let matches = item_matches(item);
        let dest = root_path.join(&item.dir);
        create_dir_all(&dest).unwrap();
        matches.iter().for_each(|f| {
            debug!(
                "{} Comparing files: {:?} | {:?}",
                pretty_item(item),
                f,
                &dest
            );
            debug!(
                "\tDestination file {}found",
                if dest.exists() { "" } else { "not " }
            );
            // TODO: strip prefix
            info!("{} Copying file: {:?} -> {:?}", pretty_item(item), f, &dest);
            let mut options = CopyOptions::new();
            options.overwrite = true;
            copy_items(&[f], &dest, &options).unwrap();
            copy_count += 1;
        })
    });
    info!(
        "Copied {} files across {} items",
        copy_count,
        config.items.len()
    )
}
