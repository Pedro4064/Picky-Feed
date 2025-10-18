pub mod rss_rule;
pub mod user_credentials;
pub mod qbittorent_config;

use rss_rule::RssRule;
use qbittorent_config::QBitTorrentConfig;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use user_credentials::UserCredentials;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub credentials: UserCredentials,
    pub qbittorrent_config: QBitTorrentConfig,
    pub rss_rules: Vec<RssRule>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigLoader {
    file_path: PathBuf,
}

impl ConfigLoader {
    pub fn new(file_path: &Path) -> ConfigLoader {
        ConfigLoader {
            file_path: file_path.to_path_buf(),
        }
    }

    pub fn parse_config(&self) -> Config {
        let mut file = File::open(self.file_path.clone())
            .expect(&format!("[ERROR] Unable to open config file"));
        let mut config = String::new();

        file.read_to_string(&mut config)
            .expect("[ERROR] Unable to read config file");

        toml::from_str::<Config>(&config).expect("[ERROR] Unable to parse config file")
    }
}
