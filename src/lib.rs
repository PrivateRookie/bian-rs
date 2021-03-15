use std::net::SocketAddr;

use bian_core::{error::APIError, BianResult};
use bian_proc::api;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use tungstenite::{
    client::{connect_with_proxy, ProxyAutoStream},
    WebSocket,
};

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

/// U 本位合约 websocket 客户端
/// [doc](https://binance-docs.github.io/apidocs/futures/cn/#websocket)
pub struct UFuturesProxyWSClient {
    pub proxy: SocketAddr,
    pub base_url: url::Url,
}

impl UFuturesProxyWSClient {
    pub fn symbol_ticker(&self, symbol: &str) -> BianResult<WebSocket<ProxyAutoStream>> {
        let url = self
            .base_url
            .join(&format!("ws/{}@ticker", symbol))
            .unwrap();
        let (socket, _) = connect_with_proxy(url, self.proxy, None, 3)
            .map_err(|e| APIError::WSConnectError(e.to_string()))?;
        Ok(socket)
    }
}

#[test]
fn test_ws() {
    use crate::response::WebsocketResponse;
    use std::net::ToSocketAddrs;

    let base_url = url::Url::parse("wss://fstream.binance.com/").unwrap();
    let proxy = "win:1087".to_socket_addrs().unwrap().next().unwrap();
    let client = UFuturesProxyWSClient { proxy, base_url };
    let mut stream = client.symbol_ticker("ethusdt").unwrap();
    loop {
        let msg: response::Ticker = stream.read_data().unwrap();
        dbg!(msg);
    }
}
