use bian_core::{error::APIError, BianResult};
use bian_proc::api;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub mod params;
pub mod response;

pub struct Client {
    http_client: reqwest::Client,
    pub api_key: String,
    pub secret_key: String,
    pub base_url: url::Url,
}

impl Client {
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

    /// 测试服务器连通性
    #[api(GET "fapi/v1/ping")]
    pub async fn ping(&self) -> BianResult<response::Ping> {}

    /// 账户余额V2
    #[api(SGET "fapi/v2/balance")]
    pub async fn account_balance_v2(
        &self,
        param: params::AccountBalanceV2,
    ) -> BianResult<Vec<response::AccountBalance>> {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    const BASE_URL: &str = "https://fapi.binance.com/";

    fn init_test() -> (String, String) {
        dotenv::dotenv().unwrap();
        let api_key = env::var("API_KEY").expect("can not find API_KEY env variable");
        let secret_key = env::var("SECRET_KEY").expect("can not find SECRET_KEY env variable");
        (api_key, secret_key)
    }

    #[tokio::test]
    async fn test_ping() {
        let (api_key, secret_key) = init_test();
        let client = Client::new(&api_key, &secret_key, BASE_URL);
        client.ping().await.unwrap();
    }

    #[tokio::test]
    async fn test_balance() {
        let (api_key, secret_key) = init_test();
        let client = Client::new(&api_key, &secret_key, BASE_URL);
        let now = chrono::Utc::now();
        let params = params::AccountBalanceV2 {
            timestamp: now.timestamp_millis(),
            recv_window: None,
        };
        client.account_balance_v2(params).await.unwrap();
    }
}
