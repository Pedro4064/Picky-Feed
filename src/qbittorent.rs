pub struct QBitTorrentClient{
    user_name: String,
    user_password: String,
    user_cookies: String,
}

impl QBitTorrentClient{
    pub fn new(user_name:String, user_password:String) -> Self{
        QBitTorrentClient {
            user_name,
            user_password
        }
    }
}