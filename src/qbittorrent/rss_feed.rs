use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article{
    id: String,
    #[serde(rename="torrentURL")]
    torrent_url: String,
    #[serde(default)]
    is_read: bool,
}

#[derive(Debug, Deserialize)]
pub struct Feed{
    articles: Vec<Article>,
}