use crate::error::{APIError, BianResult};
use crate::response;
use bian_proc::api;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub struct SpotHttpClient {
    http_client: reqwest::Client,
    pub api_key: String,
    pub secret_key: String,
    pub base_url: url::Url,
}

impl SpotHttpClient {
    pub fn new(api_key: &str, secret_key: &str, base_url: &str) -> Self {
        let http_client = reqwest::Client::new();
        Self {
            http_client,
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: url::Url::parse(base_url).unwrap(),
        }
    }

    fn sign<P: serde::Serialize>(&self, params: &P) -> String {
        let qs = serde_qs::to_string(&params).unwrap();
        let mut signed_key = Hmac::<Sha256>::new_varkey(self.secret_key.as_bytes()).unwrap();
        signed_key.update(qs.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());
        signature
    }
}

/// 行情接口
impl SpotHttpClient {
    /// 测试服务器连通性
    #[api(GET "api/v1/ping")]
    pub async fn ping(&self) -> BianResult<response::EmptyResponse> {}

    /// 获取服务器时间
    #[api(GET "api/v1/time")]
    pub async fn server_time(&self) -> BianResult<response::ServerTime> {}

    /// 获取交易规则和交易对
    #[api(GET "api/v1/exchangeInfo")]
    pub async fn exchange_info(&self) -> BianResult<response::SpotExchangeInfo> {}
}
