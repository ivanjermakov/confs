use std::fs::{copy, create_dir_all};
use std::path::Path;

use log::{debug, info, warn};

use crate::command::check::check_fatal;
use crate::config::{pretty_item, Config};
use crate::fs::{expand, item_matches};

pub fn sync(config: &Config) {
    let root_path = Path::new(&config.root);
    let mut copy_count = 0;

    check_fatal(config).and_then(|_| create_dir_all(root_path)).unwrap();

    for item in config.items.iter() {
        let item_root = expand(&item.root);
        let item_root_path = Path::new(&item_root);
        let matches = item_matches(item);
        let dest_dir = root_path.join(&item.name);

        debug!("mkdir {dest_dir:?}");
        let _ = create_dir_all(&dest_dir).inspect_err(|e| warn!("Mkdir error: {e} {dest_dir:?}"));

        for f in matches.iter() {
            if !f.exists() {
                debug!("Source file does not exist");
                break;
            }

            let dest = &dest_dir.join(f.strip_prefix(item_root_path).unwrap());
            if dest.exists() {
                debug!("Destination file exists");
            }

            if f.is_dir() {
                if !f.exists() {
                    info!("{} Creating dir: {:?}", pretty_item(item), dest);
                    debug!("mkdir {dest:?}");
                    let _ = create_dir_all(dest).inspect_err(|e| warn!("Mkdir error: {e} {dest:?}"));
                }
            } else {
                info!("{} Copying file: {:?} -> {:?}", pretty_item(item), f, dest);
                let parent_dir = dest.parent().unwrap();
                debug!("mkdir {parent_dir:?}");
                let _ = create_dir_all(parent_dir).inspect_err(|e| warn!("Mkdir error: {e} {parent_dir:?}"));
                let _ = copy(f, dest).inspect_err(|e| warn!("Copy error: {e}"));
                debug!("cp {f:?} {dest:?}");
            }
            copy_count += 1;
        }
    }
    info!("Copied {} files across {} items", copy_count, config.items.len())
}
