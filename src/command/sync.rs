use std::fs::{copy, create_dir, create_dir_all};
use std::path::Path;

use log::{debug, info, warn};

use crate::command::check::check_fatal;
use crate::config::{pretty_item, Config};
use crate::fs::{expand, item_matches};

pub fn sync(config: &Config) {
    let root_path = Path::new(&config.root);
    let mut copy_count = 0;

    check_fatal(config).and_then(|_| create_dir_all(root_path)).unwrap();

    config.items.iter().for_each(|item| {
        let item_root = expand(&item.root);
        let item_root_path = Path::new(&item_root);
        let matches = item_matches(item);
        let dest_dir = root_path.join(&item.dir);
        create_dir_all(&dest_dir).unwrap();
        matches.iter().for_each(|f| {
            if !f.exists() {
                debug!("Source file does not exist");
                return;
            }

            let dest = &dest_dir.join(f.strip_prefix(item_root_path).unwrap());
            if dest.exists() {
                debug!("Destination file exists");
            }

            if f.is_dir() {
                info!("{} Creating dir: {:?}", pretty_item(item), dest);
                match create_dir(dest) {
                    Ok(_) => {}
                    Err(e) => warn!("mkdir error: {e}"),
                }
            } else {
                info!("{} Copying file: {:?} -> {:?}", pretty_item(item), f, dest);
                match copy(f, dest) {
                    Ok(_) => {}
                    Err(e) => warn!("copy error: {e}"),
                }
            }
            copy_count += 1;
        })
    });
    info!("Copied {} files across {} items", copy_count, config.items.len())
}
