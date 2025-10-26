mod config_loader;
mod qbittorrent;

use std::path::Path;

use crate::config_loader::{Config, ConfigLoader};
fn main() {
    let config_path = Path::new("/home/pedro-cruz/Projects/picky_feed/picky_conf.toml");
    let conf_loader = ConfigLoader::new(config_path);
    let configuration = conf_loader.parse_config();
    println!("{:?}", configuration);
    // let mut torrent = qbittorent::QBitTorrentClient::new("admin".to_string(), "torrent".to_string(), "http://localhost:8080".to_string());
    let mut torrent = qbittorrent::QBitTorrentClient::new(configuration.credentials.user_name, configuration.credentials.user_password, "http://localhost:8080".to_string());
    torrent.auth_user().expect("Error in user auth, unable to proceed");
    let rss_feeds = torrent.get_rss_items().expect("Error parsing rss feeds, unable to proceed");
    println!("{:?}", rss_feeds);
}
