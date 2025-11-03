use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article{
    pub id: String,
    #[serde(rename="torrentURL")]
    pub torrent_url: String,
    #[serde(default)]
    pub is_read: bool,
}

#[derive(Debug, Deserialize)]
pub struct Feed{
    pub articles: Vec<Article>,
}