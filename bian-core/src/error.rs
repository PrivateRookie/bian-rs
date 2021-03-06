use thiserror::Error;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("4xx client side error {0}")]
    ClientSideError(String),
    #[error("403 hit web application firewall(waf)")]
    WAFLimit,
    #[error("429 frequency warning")]
    FreqWarning,
    #[error("418 banned by api")]
    Banned,
    #[error("server side error {0}")]
    ServerSideError(String),
    #[error("request error {0}")]
    RequestError(reqwest::Error),
    #[error("decode response body error {0}")]
    DecodeError(String),
    #[error("unknown {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> Self {
        APIError::RequestError(err)
    }
}

impl APIError {
    pub async fn check_resp(resp: reqwest::Response) -> Result<reqwest::Response, Self> {
        let status_code = u16::from(resp.status());
        if status_code >= 300 {
            let e = if status_code >= 500 {
                APIError::ServerSideError(resp.text().await.unwrap_or_default())
            } else if status_code == 418 {
                APIError::Banned
            } else if status_code == 429 {
                APIError::FreqWarning
            } else if status_code == 403 {
                APIError::WAFLimit
            } else if status_code >= 400 {
                APIError::ClientSideError(resp.text().await.unwrap_or_default())
            } else {
                APIError::Unknown(resp.text().await.unwrap_or_default())
            };
            Err(e)
        } else {
            Ok(resp)
        }
    }
}
