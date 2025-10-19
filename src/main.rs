mod config_loader;
mod qbittorent;

use std::path::Path;

use crate::config_loader::{Config, ConfigLoader};
fn main() {
    let config_path = Path::new("/home/pedro-cruz/Projects/picky_feed/picky_conf.toml");
    let conf_loader = ConfigLoader::new(config_path);
    let configuration = conf_loader.parse_config();
    println!("{:?}", configuration);
    qbittorent::QBitTorrentClient::new("admin".to_string(), "fzGYz4Z9M".to_string(), "http://localhost:8080".to_string());
}
