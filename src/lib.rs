use bian_core::{error::APIError, BianResult};
use bian_proc::api;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub mod params;
pub mod response;

/// U 本位合约 http 客户端
/// [doc](https://binance-docs.github.io/apidocs/futures/cn/#185368440e)
pub struct UFuturesHttpClient {
    http_client: reqwest::Client,
    pub api_key: String,
    pub secret_key: String,
    pub base_url: url::Url,
}

impl UFuturesHttpClient {
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

    /// 获取服务器时间
    #[api(GET "fapi/v1/time")]
    pub async fn server_time(&self) -> BianResult<response::ServerTime> {}

    /// 获取交易规则和交易对
    #[api(GET "fapi/v1/exchangeInfo")]
    pub async fn exchange_info(&self) -> BianResult<response::ExchangeInfo> {}

    /// 深度信息
    #[api(GET "fapi/v1/depth")]
    pub async fn depth(&self, param: params::PDepth) -> BianResult<response::Depth> {}

    /// 近期成交
    #[api(GET "fapi/v1/trades")]
    pub async fn trades(&self, param: params::PTrade) -> BianResult<Vec<response::Trade>> {}

    /// 查询历史成交
    #[api(GET "fapi/v1/historicalTrades")]
    pub async fn historical_trades(
        &self,
        param: params::PHistoricalTrade,
    ) -> BianResult<Vec<response::HistoricalTrade>> {
    }

    /// 近期成交(归集)
    #[api(GET "fapi/v1/aggTrades")]
    pub async fn agg_trades(
        &self,
        param: params::PAggTrade,
    ) -> BianResult<Vec<response::AggTrade>> {
    }

    /// K 线数据
    #[api(GET "fapi/v1/klines")]
    pub async fn klines(&self, param: params::PKline) -> BianResult<Vec<response::Kline>> {}

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
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        client.ping().await.unwrap();
    }

    #[tokio::test]
    async fn test_balance() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        let now = chrono::Utc::now();
        let params = params::AccountBalanceV2 {
            timestamp: now.timestamp_millis(),
            recv_window: None,
        };
        client.account_balance_v2(params).await.unwrap();
    }

    #[tokio::test]
    async fn test_server_time() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        client.server_time().await.unwrap();
    }
    #[tokio::test]
    async fn test_exchange_info() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        client.exchange_info().await.unwrap();
    }

    #[tokio::test]
    async fn test_depth() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        let param = params::PDepth {
            symbol: "BTCUSDT".to_string(),
            limit: 500,
        };
        dbg!(client.depth(param).await.unwrap());
    }

    #[tokio::test]
    async fn test_trades() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        let param = params::PTrade {
            symbol: "BTCUSDT".to_string(),
            limit: 500,
        };
        dbg!(client.trades(param).await.unwrap());
    }

    #[tokio::test]
    async fn test_historical_trades() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        let trade_param = params::PTrade {
            symbol: "BTCUSDT".to_string(),
            limit: 10,
        };
        let trades = client.trades(trade_param).await.unwrap();
        let htrade_param = params::PHistoricalTrade {
            symbol: "BTCUSDT".to_string(),
            from_id: Some(trades.first().unwrap().id),
            limit: None,
        };
        dbg!(client.historical_trades(htrade_param).await.unwrap());
    }

    #[tokio::test]
    async fn test_agg_trades() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        let param = params::PAggTrade {
            symbol: "BTCUSDT".to_string(),
            limit: None,
            from_id: None,
            start_time: None,
            end_time: None,
        };
        dbg!(client.agg_trades(param).await.unwrap());
    }

    #[tokio::test]
    async fn test_klines() {
        let (api_key, secret_key) = init_test();
        let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
        let param = params::PKline {
            symbol: "BTCUSDT".to_string(),
            interval: "1m".to_string(),
            start_time: None,
            end_time: None,
            limit: None,
        };
        dbg!(client.klines(param).await.unwrap());
    }
}
