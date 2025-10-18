use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileRule{
    pub name: String,
    pub file_regex: String // !Check if it is worth to deserialize as regex...
}

#[derive(Debug, Deserialize)]
pub struct RssRule{
    pub rss_name: String,
    pub download_directory: PathBuf,
    pub file_rules: Vec<FileRule>
}