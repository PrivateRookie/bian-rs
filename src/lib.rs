use std::net::SocketAddr;

use bian_proc::api;
use error::{APIError, BianResult};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use tungstenite::{
    client::{connect_with_proxy, ProxyAutoStream},
    WebSocket,
};

pub mod enums;
pub mod error;
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

    /// 连续合约K线数据
    #[api(GET "fapi/v1/continuousKlines")]
    pub async fn continuous_klines(
        &self,
        param: params::PContinuousKline,
    ) -> BianResult<Vec<response::Kline>> {
    }

    /// 价格指数K线数据
    #[api(GET "fapi/v1/indexPriceKlines")]
    pub async fn index_price_klines(
        &self,
        param: params::PIndexPriceKline,
    ) -> BianResult<Vec<response::Kline>> {
    }

    /// 最新标记价格和资金费率(单symbol)
    #[api(GET "fapi/v1/premiumIndex")]
    pub async fn premium_index(
        &self,
        param: params::PSymbol,
    ) -> BianResult<response::PremiumIndex> {
    }

    /// 最新标记价格和资金费率(所有symbol)
    #[api(GET "fapi/v1/premiumIndex")]
    pub async fn premium_indexes(&self) -> BianResult<Vec<response::PremiumIndex>> {}

    /// 查询资金费率历史
    #[api(GET "fapi/v1/fundingRate")]
    pub async fn funding_rate(
        &self,
        param: params::PFundingRate,
    ) -> BianResult<Vec<response::FundingRate>> {
    }

    /// 24小时价格变动情况(单个symbol)
    #[api(GET "fapi/v1/ticker/24hr")]
    pub async fn h24_ticker(&self, param: params::PSymbol) -> BianResult<response::H24ticker> {}

    /// 24小时价格变动情况(所有symbol)
    #[api(GET "fapi/v1/ticker/24hr")]
    pub async fn h24_tickers(&self) -> BianResult<Vec<response::H24ticker>> {}

    /// 最新价格(单个symbol)
    #[api(GET "fapi/v1/ticker/price")]
    pub async fn price(&self, param: params::PSymbol) -> BianResult<response::Price> {}

    /// 最新价格
    #[api(GET "fapi/v1/ticker/price")]
    pub async fn prices(&self) -> BianResult<Vec<response::Price>> {}

    /// 当前最优挂单(单symbol)
    #[api(GET "fapi/v1/ticker/bookTicker")]
    pub async fn book_ticker(&self, param: params::PSymbol) -> BianResult<response::BookTicker> {}

    /// 当前最优挂单
    #[api(GET "fapi/v1/ticker/bookTicker")]
    pub async fn book_tickers(&self) -> BianResult<Vec<response::BookTicker>> {}

    /// 获取市场强平订单
    ///
    /// - 如果不提供symbol,返回全市场强平订单。
    /// - 仅可查询最近7天数据
    #[api(GET "fapi/v1/allForceOrders")]
    pub async fn all_force_orders(
        &self,
        param: params::PForceOrder,
    ) -> BianResult<Vec<response::ForceOrder>> {
    }

    /// 获取未平仓合约数
    #[api(GET "fapi/v1/openInterest")]
    pub async fn open_interest(
        &self,
        param: params::PSymbol,
    ) -> BianResult<response::OpenInterest> {
    }

    /// 合约持仓量
    ///
    /// - 若无 star_time 和 end_time 限制， 则默认返回当前时间往前的limit值
    /// - 仅支持最近30天的数据
    #[api(GET "futures/data/openInterestHist")]
    pub async fn open_interest_hist(
        &self,
        param: params::PFutures,
    ) -> BianResult<Vec<response::OpenInterestHist>> {
    }

    /// 大户账户数多空比
    #[api(GET "futures/data/topLongShortAccountRatio")]
    pub async fn top_long_short_account_ratio(
        &self,
        param: params::PFutures,
    ) -> BianResult<Vec<response::LongShortRatio>> {
    }

    /// 大户持仓量多空比
    #[api(GET "futures/data/topLongShortPositionRatio")]
    pub async fn top_long_short_position_ratio(
        &self,
        param: params::PFutures,
    ) -> BianResult<Vec<response::LongShortRatio>> {
    }

    /// 多空持仓人数比
    #[api(GET "futures/data/globalLongShortAccountRatio")]
    pub async fn global_long_short_position_ratio(
        &self,
        param: params::PFutures,
    ) -> BianResult<Vec<response::LongShortRatio>> {
    }

    /// 合约主动买卖量
    #[api(GET "futures/data/takerlongshortRatio")]
    pub async fn taker_long_short_ratio(
        &self,
        param: params::PFutures,
    ) -> BianResult<Vec<response::TakerLongShortRatio>> {
    }

    /// 杠杆代币历史净值K线
    #[api(GET "fapi/v1/lvtKlines")]
    pub async fn lvt_klines(&self, param: params::PLvtKlines) -> BianResult<Vec<response::Kline>> {}

    /// 综合指数交易对信息(单个)
    #[api(GET "fapi/v1/indexInfo")]
    pub async fn index_info(&self, param: params::PSymbol) -> BianResult<response::IndexInfo> {}

    /// 综合指数交易对信息
    #[api(GET "fapi/v1/indexInfo")]
    pub async fn index_infos(&self) -> BianResult<Vec<response::IndexInfo>> {}

    /// 账户余额V2
    #[api(SGET "fapi/v2/balance")]
    pub async fn account_balance_v2(
        &self,
        param: params::AccountBalanceV2,
    ) -> BianResult<Vec<response::AccountBalance>> {
    }
}

/// U 本位合约 websocket 客户端(使用代理)
/// [doc](https://binance-docs.github.io/apidocs/futures/cn/#websocket)
pub struct UFuturesProxyWSClient {
    pub proxy: SocketAddr,
    pub base_url: url::Url,
}

impl UFuturesProxyWSClient {
    fn build_single(
        &self,
        symbol: String,
        channel: &str,
    ) -> BianResult<WebSocket<ProxyAutoStream>> {
        let url = self
            .base_url
            .join(&format!("ws/{}@{}", symbol, channel))
            .unwrap();
        let (socket, _) = connect_with_proxy(url, self.proxy, None, 3)
            .map_err(|e| APIError::WSConnectError(e.to_string()))?;
        Ok(socket)
    }

    fn build_multi(
        &self,
        symbols: Vec<String>,
        channel: &str,
    ) -> BianResult<WebSocket<ProxyAutoStream>> {
        let streams = symbols
            .iter()
            .map(|sym| format!("{}@{}", sym, channel))
            .collect::<Vec<String>>()
            .join("/");
        let url = self
            .base_url
            .join(&format!("stream/?streams={}", streams))
            .unwrap();
        let (socket, _) = connect_with_proxy(url, self.proxy, None, 3)
            .map_err(|e| APIError::WSConnectError(e.to_string()))?;
        Ok(socket)
    }

    /// 同一价格、同一方向、同一时间(100ms计算)的trade会被聚合为一条
    pub fn agg_trade(&self, symbol: String) -> BianResult<WebSocket<ProxyAutoStream>> {
        self.build_single(symbol, "aggTrade")
    }

    /// 同一价格、同一方向、同一时间(100ms计算)的trade会被聚合为一条
    pub fn agg_trade_multi(&self, symbols: Vec<String>) -> BianResult<WebSocket<ProxyAutoStream>> {
        self.build_multi(symbols, "aggTrade")
    }

    /// 按Symbol刷新的24小时完整ticker信息
    pub fn symbol_ticker(&self, symbol: String) -> BianResult<WebSocket<ProxyAutoStream>> {
        self.build_single(symbol, "ticker")
    }

    /// 按Symbol刷新的24小时完整ticker信息
    pub fn symbol_ticker_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<WebSocket<ProxyAutoStream>> {
        self.build_multi(symbols, "ticker")
    }
}

/// U 本位合约 websocket 客户端
/// [doc](https://binance-docs.github.io/apidocs/futures/cn/#websocket)
pub struct UFuturesWSClient {
    pub base_url: url::Url,
}
