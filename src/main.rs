mod config_loader;

use std::path::Path;

use crate::config_loader::{Config, ConfigLoader};
fn main() {
    let config_path = Path::new("/home/pedro-cruz/Projects/picky_feed/picky_conf.toml");
    let conf_loader = ConfigLoader::new(config_path);
    let configuration = conf_loader.parse_config();
    println!("{:?}", configuration);
}
