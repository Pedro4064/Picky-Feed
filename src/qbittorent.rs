pub mod api_endpoints;
pub mod api_errors;

use api_errors::QbitApiError;
use reqwest::{header::HeaderMap, Client};

pub struct QBitTorrentClient {
    user_name: String,
    user_password: String,
    user_cookies: String,
    service_host: String,
}

impl QBitTorrentClient {
    pub fn get_session_cookie(
        user_name: String,
        user_password: String,
        host: String,
    ) -> Result<String, QbitApiError> {
        let mut headers = HeaderMap::new();
        headers.insert("Referer", host.parse().unwrap());
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut params = std::collections::HashMap::new();
        params.insert("username", &user_name);
        params.insert("password", &user_password);

        let request = reqwest::blocking::Client::new();
        println!("{}", format!("{}{}", host, api_endpoints::LOGIN));
        let res = request
            .post(format!("{}{}", host, api_endpoints::LOGIN))
            .headers(headers)
            .form(&params)
            .send();

        let res = match res {
            Ok(response) => response,
            Err(_) => return Err(QbitApiError::FailedEndpoint(api_endpoints::LOGIN)),
        };

        let cookie_result = match res.cookies().next(){
            Some(val) => Ok(val.value().to_string()),
            None => Err(QbitApiError::FailedAuth)
        };

        cookie_result
    }

    pub fn new(user_name: String, user_password: String, host: String) -> Self {
        // QBitTorrentClient {
        // user_name,
        // user_password,
        // }
        let cookie = QBitTorrentClient::get_session_cookie(user_name, user_password, host);
        println!("{:?}", cookie);
        todo!();
    }

    pub fn auth_user(&self) -> Result<(), QbitApiError> {
        // let cookie = QBitTorrentClient::get_session_cookie(user_name, user_password, host);
        todo!();
    }
}
