mod config_loader;
mod qbittorent;

use std::path::Path;

use crate::config_loader::{Config, ConfigLoader};
fn main() {
    let config_path = Path::new("/home/pedro-cruz/Projects/picky_feed/picky_conf.toml");
    let conf_loader = ConfigLoader::new(config_path);
    let configuration = conf_loader.parse_config();
    println!("{:?}", configuration);
    // let mut torrent = qbittorent::QBitTorrentClient::new("admin".to_string(), "torrent".to_string(), "http://localhost:8080".to_string());
    let mut torrent = qbittorent::QBitTorrentClient::new(configuration.credentials.user_name, configuration.credentials.user_password, "http://localhost:8080".to_string());
    torrent.auth_user();
    torrent.get_rss_items();
}
