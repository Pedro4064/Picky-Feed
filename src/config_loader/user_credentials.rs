use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCredentials{
    pub user_name:String,
    pub user_password:String
}