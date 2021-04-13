use std::net::SocketAddr;

use crate::{
    enums::Interval,
    error::{APIError, BianResult},
    response::WebsocketResponse,
};
use crate::{params, response};
use bian_proc::api;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use tungstenite::{
    client::{connect_with_config, AutoGenericStream},
    WebSocket,
};

/// 现货账户客户端
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
    #[api(GET "api/v3/ping")]
    pub async fn ping(&self) -> BianResult<response::EmptyResponse> {}

    /// 获取服务器时间
    #[api(GET "api/v3/time")]
    pub async fn server_time(&self) -> BianResult<response::ServerTime> {}

    /// 获取交易规则和交易对
    #[api(GET "api/v3/exchangeInfo")]
    pub async fn exchange_info(&self) -> BianResult<response::SpotExchangeInfo> {}

    #[api(GET "api/v3/depth")]
    pub async fn depth(&self, param: params::PDepth) -> BianResult<response::SpotDepth> {}

    /// 近期成交
    #[api(GET "api/v3/trades")]
    pub async fn trades(&self, param: params::PTrade) -> BianResult<Vec<response::SpotTrade>> {}

    /// 查询历史成交
    #[api(GET "api/v3/historicalTrades")]
    pub async fn historical_trades(
        &self,
        param: params::PHistoricalTrade,
    ) -> BianResult<Vec<response::SpotHistoricalTrade>> {
    }

    /// 近期成交(归集)
    #[api(GET "api/v3/aggTrades")]
    pub async fn agg_trades(
        &self,
        param: params::PAggTrade,
    ) -> BianResult<Vec<response::AggTrade>> {
    }

    /// K 线数据
    #[api(GET "api/v3/klines")]
    pub async fn klines(&self, param: params::PKline) -> BianResult<Vec<response::Kline>> {}

    /// 当前平均价格
    #[api(GET "api/v3/avgPrice")]
    pub async fn avg_price(&self, param: params::PSymbol) -> BianResult<response::AvgPrice> {}

    /// 24小时价格变动情况(单个symbol)
    #[api(GET "api/v3/ticker/24hr")]
    pub async fn h24_ticker(&self, param: params::PSymbol) -> BianResult<response::H24ticker> {}

    /// 24小时价格变动情况(所有symbol)
    #[api(GET "api/v3/ticker/24hr")]
    pub async fn h24_tickers(&self) -> BianResult<Vec<response::H24ticker>> {}

    /// 最新价格(单个symbol)
    #[api(GET "api/v3/ticker/price")]
    pub async fn price(&self, param: params::PSymbol) -> BianResult<response::SpotPrice> {}

    /// 最新价格
    #[api(GET "api/v3/ticker/price")]
    pub async fn prices(&self) -> BianResult<Vec<response::SpotPrice>> {}

    /// 当前最优挂单(单symbol)
    #[api(GET "api/v3/ticker/bookTicker")]
    pub async fn book_ticker(
        &self,
        param: params::PSymbol,
    ) -> BianResult<response::SpotBookTicker> {
    }

    /// 当前最优挂单
    #[api(GET "api/v3/ticker/bookTicker")]
    pub async fn book_tickers(&self) -> BianResult<Vec<response::SpotBookTicker>> {}
}

/// 现货账户和交易接口
impl SpotHttpClient {
    /// 下单
    #[api(SPOST "api/v3/order")]
    pub async fn order(&self, param: params::PSpotOrder) -> BianResult<response::SpotOrder> {}

    /// 撤销订单
    #[api(SDELETE "api/v3/order")]
    pub async fn cancel_order(
        &self,
        param: params::PQuerySpotOrder,
    ) -> BianResult<response::SpotOpOrder> {
    }

    /// 撤销单一交易对的所有挂单
    ///
    /// 撤销单一交易对下所有挂单, 包括OCO的挂单。
    #[api(SDELETE "api/v3/openOrders")]
    pub async fn cancel_open_orders(
        &self,
        param: params::PSymbolWithTs,
    ) -> BianResult<Vec<response::CancelSpotOrder>> {
    }

    /// 查询订单
    #[api(SGET "api/v3/order")]
    pub async fn list_order(
        &self,
        param: params::PQuerySpotOrder,
    ) -> BianResult<response::QuerySpotOrder> {
    }

    /// 当前挂单
    #[api(SGET "api/v3/openOrders")]
    pub async fn open_orders(
        &self,
        param: params::POptionSymbolQuery,
    ) -> BianResult<Vec<response::QuerySpotOrder>> {
    }

    /// 查询所有订单
    ///
    /// 获取所有帐户订单； 有效，已取消或已完成。
    #[api(SGEt "api/v3/allOrders")]
    pub async fn list_all_order(
        &self,
        param: params::PQueryAllSpotOrder,
    ) -> BianResult<Vec<response::QuerySpotOrder>> {
    }

    /// OCO 下单
    /// [DOC](https://binance-docs.github.io/apidocs/spot/cn/#oco-trade)
    #[api(SPOST "api/v3/order/oco")]
    pub async fn oco_order(&self, param: params::POcoOrder) -> BianResult<response::OcoOrder> {}

    /// 取消 OCO 订单
    #[api(SDELETE "api/v3/orderList")]
    pub async fn cancel_oco_order(
        &self,
        param: params::PCancelOcoOrder,
    ) -> BianResult<response::OcoOrder> {
    }

    /// 查询 OCO
    #[api(SGET "api/v3/orderList")]
    pub async fn list_oco_order(
        &self,
        param: params::PQueryOcoOrder,
    ) -> BianResult<response::OcoOrder> {
    }

    /// 查询所有 OCO
    #[api(SGET "api/v3/allOrderList")]
    pub async fn list_all_oco_order(
        &self,
        param: params::PQueryAllOcoOrder,
    ) -> BianResult<Vec<response::OcoOrder>> {
    }

    /// 查询 OCO 挂单
    #[api(SGET "api/v3/openOrderList")]
    pub async fn list_open_oco_order(
        &self,
        param: params::PTimestamp,
    ) -> BianResult<Vec<response::OcoOrder>> {
    }

    /// 账户信息
    #[api(SGET "api/v3/account")]
    pub async fn account(&self, param: params::PTimestamp) -> BianResult<response::SpotAccount> {}

    /// 账户成交历史
    #[api(SGET "api/v3/myTrades")]
    pub async fn my_trades(
        &self,
        param: params::PUserTrade,
    ) -> BianResult<Vec<response::UserSpotTrade>> {
    }
}

/// 现货 websocket 客户端
pub struct SpotWSClient {
    pub proxy: Option<SocketAddr>,
    pub base_url: url::Url,
}

impl SpotWSClient {
    fn build_single(
        &self,
        symbol: String,
        channel: &str,
    ) -> BianResult<WebSocket<AutoGenericStream>> {
        let url = if symbol.is_empty() {
            self.base_url.join(&format!("ws/{}", channel)).unwrap()
        } else {
            self.base_url
                .join(&format!("ws/{}@{}", symbol, channel))
                .unwrap()
        };
        let (socket, _) = connect_with_config(url, None, 3, self.proxy)
            .map_err(|e| APIError::WSConnectError(e.to_string()))?;
        Ok(socket)
    }

    fn build_multi(
        &self,
        symbols: Vec<String>,
        channel: &str,
    ) -> BianResult<WebSocket<AutoGenericStream>> {
        let streams = symbols
            .iter()
            .map(|sym| format!("{}@{}", sym, channel))
            .collect::<Vec<String>>()
            .join("/");
        let url = self
            .base_url
            .join(&format!("stream/?streams={}", streams))
            .unwrap();
        let (socket, _) = connect_with_config(url, None, 3, self.proxy)
            .map_err(|e| APIError::WSConnectError(e.to_string()))?;
        Ok(socket)
    }
}

/// 行情
impl SpotWSClient {
    /// 归集交易 stream 推送交易信息，是对单一订单的集合.
    pub fn agg_trade(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSAggTrade>> {
        self.build_single(symbol, "aggTrade")
    }

    /// 归集交易 stream 推送交易信息，是对单一订单的集合.
    pub fn agg_trade_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSAggTrade>> {
        self.build_multi(symbols, "aggTrade")
    }

    /// 逐笔交易
    pub fn trade(&self, symbol: String) -> BianResult<impl WebsocketResponse<response::WSTrade>> {
        self.build_single(symbol, "trade")
    }

    /// 逐笔交易
    pub fn trade_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSTrade>> {
        self.build_multi(symbols, "trade")
    }

    // K线
    ///
    /// K线stream逐秒推送所请求的K线种类(最新一根K线)的更新。推送间隔2000毫秒(如有刷新)
    pub fn kline(
        &self,
        symbol: String,
        interval: Interval,
    ) -> BianResult<impl WebsocketResponse<response::WSKline>> {
        let channel = format!("kline_{}", interval.to_string());
        dbg!(&channel);
        self.build_single(symbol, &channel)
    }

    /// K线
    ///
    /// K线stream逐秒推送所请求的K线种类(最新一根K线)的更新。推送间隔2000毫秒(如有刷新)
    pub fn kline_multi(
        &self,
        symbols: Vec<String>,
        interval: Interval,
    ) -> BianResult<impl WebsocketResponse<response::WSKline>> {
        let channel = format!("kline_{}", interval.to_string());
        dbg!(&channel);
        self.build_multi(symbols, &channel)
    }

    /// 按 symbol 的精简 Ticker
    ///
    /// Update Speed: 1000ms
    pub fn mini_ticker(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSMiniTicker>> {
        self.build_single(symbol, "miniTicker")
    }

    /// 按 symbol 的精简 Ticker
    ///
    /// Update Speed: 1000ms
    pub fn mini_ticker_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSMiniTicker>> {
        self.build_multi(symbols, "miniTicker")
    }

    /// 全市场的精简 Ticker
    ///
    /// 所有symbol24小时精简ticker信息.需要注意的是，只有发生变化的ticker更新才会被推送。
    /// Update Speed: 1000ms
    pub fn all_mini_ticker(
        &self,
    ) -> BianResult<impl WebsocketResponse<Vec<response::WSMiniTicker>>> {
        self.build_single(String::new(), "!miniTicker@arr")
    }

    // TODO fix ticker field
    /// 按Symbol刷新的24小时完整ticker信息
    ///
    /// Update Speed: 1000ms
    pub fn symbol_ticker(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSTicker>> {
        self.build_single(symbol, "ticker")
    }

    /// 按Symbol刷新的24小时完整ticker信息
    ///
    /// Update Speed: 1000ms
    pub fn symbol_ticker_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSTicker>> {
        self.build_multi(symbols, "ticker")
    }

    /// 全市场的完整Ticker
    ///
    /// 所有symbol 24小时完整ticker信息.需要注意的是，只有发生变化的ticker更新才会被推送。
    /// Update Speed: 1000ms
    pub fn all_symbol_ticker(&self) -> BianResult<impl WebsocketResponse<Vec<response::WSTicker>>> {
        self.build_single(String::new(), "!ticker@arr")
    }
}
