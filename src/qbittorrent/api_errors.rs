use std::fmt;

#[derive(Debug, Clone)]
pub enum QbitApiError {
    FailedEndpoint(&'static str),
    FailedAuth,
    FailedRssFeedCheck,
    FailedTorrentUpload,
}


impl fmt::Display for QbitApiError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            QbitApiError::FailedEndpoint(endpoint) => write!(f, "Failed the request to {}",endpoint),
            QbitApiError::FailedAuth => write!(f, "Failed to Authenticate user"),
            QbitApiError::FailedRssFeedCheck => write!(f, "Failed to parse server response for RSS feed items"),
            QbitApiError::FailedTorrentUpload => write!(f, "Failed to upload torrent to client"),
        }

    }
}