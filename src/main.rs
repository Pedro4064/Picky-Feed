mod config_loader;
mod qbittorrent;

use std::path::Path;

use crate::{
    config_loader::{Config, ConfigLoader},
    qbittorrent::rss_feed::Article,
};
fn main() {
    let config_path = Path::new("/home/pedro-cruz/Projects/picky_feed/picky_conf.toml");
    let conf_loader = ConfigLoader::new(config_path);
    let configuration = conf_loader.parse_config();
    println!("{:?}", configuration);

    let mut torrent = qbittorrent::QBitTorrentClient::new(
        configuration.credentials.user_name,
        configuration.credentials.user_password,
        "http://localhost:8080".to_string(),
    );
    torrent
        .auth_user()
        .expect("Error in user auth, unable to proceed");
    let rss_feeds = torrent
        .get_rss_items()
        .expect("Error parsing rss feeds, unable to proceed");

    for rss_feed_config in configuration.rss_rules {
        let rss_feed_items = match rss_feeds.get(&rss_feed_config.rss_name) {
            Some(items) => items,
            None => {
                println!("No rss feed found with name {}", &rss_feed_config.rss_name);
                continue;
            }
        };

        let rss_unread_items: Vec<&Article> = rss_feed_items
            .articles
            .iter()
            .filter(|item| !item.is_read)
            .collect();

        for articles in &rss_unread_items {
            match torrent.upload_torrent_from_url(&articles.torrent_url){
                Ok(_) => (),
                Err(err) => eprintln!("{}",err)
            }
            break;
        }
        println!("{:?}", rss_unread_items);
    }
}
