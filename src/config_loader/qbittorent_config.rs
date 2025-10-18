use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QBitTorrentConfig{
    pub host:String,
    pub port:String
}