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
    pub fn get_session_cookie(&self) -> Result<String, QbitApiError> {
        let mut headers = HeaderMap::new();
        headers.insert("Referer", self.service_host.parse().unwrap());
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut params = std::collections::HashMap::new();
        params.insert("username", &self.user_name);
        params.insert("password", &self.user_password);

        let request = reqwest::blocking::Client::new();
        println!(
            "{}",
            format!("{}{}", self.service_host, api_endpoints::LOGIN)
        );
        let res = request
            .post(format!("{}{}", self.service_host, api_endpoints::LOGIN))
            .headers(headers)
            .form(&params)
            .send();

        let res = match res {
            Ok(response) => response,
            Err(_) => return Err(QbitApiError::FailedEndpoint(api_endpoints::LOGIN)),
        };

        let cookie_result = match res.cookies().next() {
            Some(val) => Ok(val.value().to_string()),
            None => Err(QbitApiError::FailedAuth),
        };

        cookie_result
    }

    pub fn new(user_name: String, user_password: String, service_host: String) -> Self {
        QBitTorrentClient {
            user_name,
            user_password,
            user_cookies: String::new(),
            service_host,
        }
    }

    pub fn auth_user(&mut self) -> Result<(), QbitApiError> {
        let cookie = QBitTorrentClient::get_session_cookie(self);
        println!("{:?}", cookie);
        self.user_cookies = cookie?;
        Ok(())
    }
}
