use std::net::SocketAddr;

use crate::enums::{ContractType, Interval};
use crate::error::{APIError, BianResult};
use crate::params;
use crate::response::{self, WebsocketResponse};
use bian_proc::api;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use tungstenite::{
    client::{connect_with_config, AutoGenericStream},
    WebSocket,
};

const BASE_HTTP_URL: &str = "https://dapi.binance.com";
const BASE_WS_URL: &str = "wss://dstream.binance.com";

/// U 本位合约 http 客户端
/// [doc](https://binance-docs.github.io/apidocs/futures/cn/#185368440e)
#[derive(Debug, Clone)]
pub struct UFuturesHttpClient {
    http_client: reqwest::Client,
    pub api_key: String,
    pub secret_key: String,
    pub base_url: url::Url,
}

impl UFuturesHttpClient {
    /// create client from default endpoint url
    pub fn default_endpoint(api_key: String, secret_key: String) -> Self {
        let base_url = url::Url::parse(BASE_HTTP_URL).unwrap();
        let http_client = reqwest::Client::new();
        Self {
            base_url,
            api_key,
            secret_key,
            http_client,
        }
    }

    pub fn new(api_key: String, secret_key: String, base_url: &str) -> BianResult<Self> {
        let http_client = reqwest::Client::new();
        let base_url = url::Url::parse(base_url)
            .map_err(|_| crate::error::APIError::InvalidUrl(base_url.to_string()))?;
        Ok(Self {
            http_client,
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url,
        })
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
impl UFuturesHttpClient {
    /// 测试服务器连通性
    #[api(GET "fapi/v1/ping")]
    pub async fn ping(&self) -> BianResult<response::EmptyResponse> {}

    /// 获取服务器时间
    #[api(GET "fapi/v1/time")]
    pub async fn server_time(&self) -> BianResult<response::ServerTime> {}

    /// 获取交易规则和交易对
    #[api(GET "fapi/v1/exchangeInfo")]
    pub async fn exchange_info(&self) -> BianResult<response::UFuturesExchangeInfo> {}

    /// 深度信息
    #[api(GET "fapi/v1/depth")]
    pub async fn depth(&self, param: params::PDepth) -> BianResult<response::FuturesDepth> {}

    /// 近期成交
    #[api(GET "fapi/v1/trades")]
    pub async fn trades(&self, param: params::PTrade) -> BianResult<Vec<response::UFuturesTrade>> {}

    /// 查询历史成交
    #[api(GET "fapi/v1/historicalTrades")]
    pub async fn historical_trades(
        &self,
        param: params::PHistoricalTrade,
    ) -> BianResult<Vec<response::UFuturesHistoricalTrade>> {
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
    ) -> BianResult<response::UPremiumIndex> {
    }

    /// 最新标记价格和资金费率(所有symbol)
    #[api(GET "fapi/v1/premiumIndex")]
    pub async fn premium_indexes(&self) -> BianResult<Vec<response::UPremiumIndex>> {}

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
    pub async fn price(&self, param: params::PSymbol) -> BianResult<response::FuturesPrice> {}

    /// 最新价格
    #[api(GET "fapi/v1/ticker/price")]
    pub async fn prices(&self) -> BianResult<Vec<response::FuturesPrice>> {}

    /// 当前最优挂单(单symbol)
    #[api(GET "fapi/v1/ticker/bookTicker")]
    pub async fn book_ticker(
        &self,
        param: params::PSymbol,
    ) -> BianResult<response::UFuturesBookTicker> {
    }

    /// 当前最优挂单
    #[api(GET "fapi/v1/ticker/bookTicker")]
    pub async fn book_tickers(&self) -> BianResult<Vec<response::UFuturesBookTicker>> {}

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
    ) -> BianResult<response::UOpenInterest> {
    }

    /// 合约持仓量
    ///
    /// - 若无 star_time 和 end_time 限制， 则默认返回当前时间往前的limit值
    /// - 仅支持最近30天的数据
    #[api(GET "futures/data/openInterestHist")]
    pub async fn open_interest_hist(
        &self,
        param: params::PUFutures,
    ) -> BianResult<Vec<response::UOpenInterestHist>> {
    }

    /// 大户账户数多空比
    #[api(GET "futures/data/topLongShortAccountRatio")]
    pub async fn top_long_short_account_ratio(
        &self,
        param: params::PUFutures,
    ) -> BianResult<Vec<response::LongShortRatio>> {
    }

    /// 大户持仓量多空比
    #[api(GET "futures/data/topLongShortPositionRatio")]
    pub async fn top_long_short_position_ratio(
        &self,
        param: params::PUFutures,
    ) -> BianResult<Vec<response::LongShortRatio>> {
    }

    /// 多空持仓人数比
    #[api(GET "futures/data/globalLongShortAccountRatio")]
    pub async fn global_long_short_position_ratio(
        &self,
        param: params::PUFutures,
    ) -> BianResult<Vec<response::LongShortRatio>> {
    }

    /// 合约主动买卖量
    #[api(GET "futures/data/takerlongshortRatio")]
    pub async fn taker_long_short_ratio(
        &self,
        param: params::PUFutures,
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
}

/// 账户和交易接口
impl UFuturesHttpClient {
    /// 更改持仓模式
    ///
    /// 变换用户在 所有symbol 合约上的持仓模式：双向持仓或单向持仓。
    #[api(SPOST "fapi/v1/positionSide/dual")]
    pub async fn update_position_side(
        &self,
        param: params::PPositionSideDual,
    ) -> BianResult<response::CodeResponse> {
    }

    ///查询持仓模式
    #[api(SGET "fapi/v1/positionSide/dual")]
    pub async fn get_position_side(
        &self,
        param: params::PTimestamp,
    ) -> BianResult<response::PositionSide> {
    }

    /// 下单
    /// [DOC](https://binance-docs.github.io/apidocs/futures/cn/#trade-2)
    #[api(SPOST "fapi/v1/order")]
    pub async fn order(&self, param: params::PFuturesOrder) -> BianResult<response::FuturesOrder> {}

    /// 批量下单
    /// [DOC](https://binance-docs.github.io/apidocs/futures/cn/#trade-4)
    #[api(SPOST "fapi/v1/batchOrders")]
    pub async fn batch_order(
        &self,
        param: params::PBatchOrder,
    ) -> BianResult<Vec<response::BatchOrderResponse>> {
    }

    /// 查询订单
    ///
    /// 至少需要发送 orderId 与 origClientOrderId 中的一个
    #[api(SGET "fapi/v1/order")]
    pub async fn query_order(
        &self,
        param: params::PQueryFuturesOrder,
    ) -> BianResult<response::FuturesOrder> {
    }

    /// 撤销订单
    ///
    /// 至少需要发送 orderId 与 origClientOrderId 中的一个
    #[api(SDELETE "fapi/v1/order")]
    pub async fn cancel_order(
        &self,
        param: params::PQueryFuturesOrder,
    ) -> BianResult<response::FuturesOrder> {
    }

    /// 撤销全部订单
    #[api(SDELETE "fapi/v1/allOpenOrders")]
    pub async fn cancel_all_orders(
        &self,
        param: params::PSymbolWithTs,
    ) -> BianResult<response::CodeResponse> {
    }

    /// 批量撤销订单
    #[api(SDELETE "fapi/v1/batchOrders")]
    pub async fn batch_cancel_orders(
        &self,
        param: params::PBatchCancelOrder,
    ) -> BianResult<Vec<response::BatchOrderResponse>> {
    }

    /// 倒计时撤销所有订单
    /// [DOC](https://binance-docs.github.io/apidocs/futures/cn/#trade-8)
    #[api(SPOST "fapi/v1/countdownCancelAll")]
    pub async fn countdown_cancel_all(
        &self,
        param: params::PCountdownCancel,
    ) -> BianResult<response::CountdownCancel> {
    }

    /// 查询当前挂单
    #[api(SGET "fapi/v1/openOrder")]
    pub async fn open_order(
        &self,
        param: params::PQueryFuturesOrder,
    ) -> BianResult<response::FuturesOrder> {
    }

    /// 查询当前全部挂单
    #[api(SGET "fapi/v1/openOrders")]
    pub async fn open_orders(
        &self,
        param: params::POptionSymbolQuery,
    ) -> BianResult<Vec<response::FuturesOrder>> {
    }

    /// 查询所有订单(包括历史订单)
    ///
    /// - 查询时间范围最大不得超过7天
    /// - 默认查询最近7天内的数据
    #[api(SGET "fapi/v1/allOrders")]
    pub async fn all_orders(
        &self,
        param: params::PHistOrder,
    ) -> BianResult<Vec<response::FuturesOrder>> {
    }

    /// 账户余额V2
    #[api(SGET "fapi/v2/balance")]
    pub async fn account_balance_v2(
        &self,
        param: params::PTimestamp,
    ) -> BianResult<Vec<response::UFuturesAccountBalance>> {
    }

    /// 账户信息v2
    #[api(SGET "fapi/v2/account")]
    pub async fn account_v2(
        &self,
        param: params::PTimestamp,
    ) -> BianResult<response::FuturesAccount> {
    }

    /// 调整开仓杠杆
    #[api(SPOST "fapi/v1/leverage")]
    pub async fn leverage(&self, param: params::PLeverage) -> BianResult<response::Leverage> {}

    /// 变换逐全仓模式
    #[api(SPOST "fapi/v1/marginType")]
    pub async fn margin_type(
        &self,
        param: params::PMarginType,
    ) -> BianResult<response::CodeResponse> {
    }

    /// 调整逐仓保证金
    #[api(SPOST "fapi/v1/positionMargin")]
    pub async fn position_margin(
        &self,
        param: params::PPositionMargin,
    ) -> BianResult<response::PositionMargin> {
    }

    /// 逐仓保证金变动历史
    #[api(SGET "fapi/v1/positionMargin/history")]
    pub async fn position_margin_history(
        &self,
        param: params::PPositionMarginHist,
    ) -> BianResult<Vec<response::PositionMarginHist>> {
    }

    /// 用户持仓风险
    ///
    /// 请与账户推送信息ACCOUNT_UPDATE配合使用，以满足您的及时性和准确性需求。
    #[api(SGET "fapi/v2/positionRisk")]
    pub async fn position_risk(
        &self,
        param: params::PSymbolWithTs,
    ) -> BianResult<Vec<response::PositionRisk>> {
    }

    /// 账户成交历史
    #[api(SGET "fapi/v1/userTrades")]
    pub async fn user_trades(
        &self,
        param: params::PUserTrade,
    ) -> BianResult<Vec<response::UserFuturesTrade>> {
    }
}

/// websocket 相关接口
impl UFuturesHttpClient {
    /// 生成 listenKey
    ///
    /// 创建一个新的user data stream，返回值为一个listenKey，即websocket订阅的stream名称。
    /// 如果该帐户具有有效的listenKey，则将返回该listenKey并将其有效期延长60分钟。
    #[api(SPOST "fapi/v1/listenKey")]
    pub async fn create_listen_key(&self) -> BianResult<response::ListenKey> {}

    /// 更新 listenKey
    #[api(SPUT "fapi/v1/listenKey")]
    pub async fn update_listen_key(&self) -> BianResult<response::EmptyResponse> {}

    /// 关闭 listenKey
    #[api(SDELETE "fapi/v1/listenKey")]
    pub async fn close_listen_key(&self) -> BianResult<response::EmptyResponse> {}
}

/// U 本位合约 websocket 客户端
/// [doc](https://binance-docs.github.io/apidocs/futures/cn/#websocket)
#[derive(Debug, Clone)]
pub struct UFuturesWSClient {
    pub proxy: Option<SocketAddr>,
    pub base_url: url::Url,
}

impl UFuturesWSClient {
    /// create client from default endpoint url
    pub fn default_endpoint(proxy: Option<SocketAddr>) -> Self {
        let base_url = url::Url::parse(BASE_WS_URL).unwrap();
        Self { base_url, proxy }
    }

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
            .join(&format!("stream?streams={}", streams))
            .unwrap();
        let (socket, _) = connect_with_config(url, None, 3, self.proxy)
            .map_err(|e| APIError::WSConnectError(e.to_string()))?;
        Ok(socket)
    }
}

/// 行情
impl UFuturesWSClient {
    /// 同一价格、同一方向、同一时间(100ms计算)的trade会被聚合为一条
    pub fn agg_trade(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSAggTrade>> {
        self.build_single(symbol, "aggTrade")
    }

    /// 同一价格、同一方向、同一时间(100ms计算)的trade会被聚合为一条
    pub fn agg_trade_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSAggTrade>> {
        self.build_multi(symbols, "aggTrade")
    }

    /// 最新标记价格
    ///
    /// freq == 1 时更新速度为1s, 否则为3s
    pub fn mark_price(
        &self,
        symbol: String,
        freq: usize,
    ) -> BianResult<impl WebsocketResponse<response::WSPrice>> {
        let channel = if freq == 1 {
            "markPrice@1s"
        } else {
            "markPrice"
        };
        self.build_single(symbol, channel)
    }

    /// 最新标记价格
    ///
    /// freq == 1 时更新速度为1s, 否则为3s
    pub fn mark_price_multi(
        &self,
        symbols: Vec<String>,
        freq: usize,
    ) -> BianResult<impl WebsocketResponse<response::WSPrice>> {
        let channel = if freq == 1 {
            "markPrice@1s"
        } else {
            "markPrice"
        };
        self.build_multi(symbols, channel)
    }

    /// 全市场最新标记价格
    ///
    /// freq == 1 时更新速度为1s, 否则为3s
    pub fn mark_price_arr(
        &self,
        freq: usize,
    ) -> BianResult<impl WebsocketResponse<Vec<response::WSPrice>>> {
        let channel = if freq == 1 {
            "!markPrice@arr@1s"
        } else {
            "!markPrice@arr"
        };
        self.build_single("".to_string(), channel)
    }

    /// K线
    ///
    /// K线stream逐秒推送所请求的K线种类(最新一根K线)的更新。推送间隔250毫秒(如有刷新)
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
    /// K线stream逐秒推送所请求的K线种类(最新一根K线)的更新。推送间隔250毫秒(如有刷新)
    pub fn kline_multi(
        &self,
        symbols: Vec<String>,
        interval: Interval,
    ) -> BianResult<impl WebsocketResponse<response::WSKline>> {
        let channel = format!("kline_{}", interval.to_string());
        dbg!(&channel);
        self.build_multi(symbols, &channel)
    }

    /// 连续合约K线
    ///
    /// K线stream逐秒推送所请求的K线种类(最新一根K线)的更新。Update Speed: 250ms
    ///
    /// 允许的 contract_type
    /// - PERPETUAL 永续合约
    /// - CURRENT_MONTH 当月交割合约
    /// - NEXT_MONTH 次月交割合约
    pub fn continuous_kline(
        &self,
        pair: String,
        contract_type: ContractType,
        interval: Interval,
    ) -> BianResult<impl WebsocketResponse<response::WSContinuousKline>> {
        let symbol = dbg!(format!("{}_{}", pair, contract_type.to_string()));
        let channel = format!("continuousKline_{}", interval.to_string());
        self.build_single(symbol, &channel)
    }

    /// 连续合约K线
    ///
    /// K线stream逐秒推送所请求的K线种类(最新一根K线)的更新。Update Speed: 250ms
    ///
    /// 允许的 contract_type
    /// - PERPETUAL 永续合约
    /// - CURRENT_MONTH 当月交割合约
    /// - NEXT_MONTH 次月交割合约
    pub fn continuous_kline_multi(
        &self,
        pairs: Vec<String>,
        contract_type: ContractType,
        interval: Interval,
    ) -> BianResult<impl WebsocketResponse<response::WSContinuousKline>> {
        let symbols = pairs
            .into_iter()
            .map(|p| format!("{}_{}", p, contract_type.to_string()))
            .collect();
        let channel = format!("continuousKline_{}", interval.to_string());
        self.build_single(symbols, &channel)
    }

    /// 按 symbol 的精简 Ticker
    ///
    /// Update Speed: 500ms
    pub fn mini_ticker(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSMiniTicker>> {
        self.build_single(symbol, "miniTicker")
    }

    /// 按 symbol 的精简 Ticker
    ///
    /// Update Speed: 500ms
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

    /// 按Symbol刷新的24小时完整ticker信息
    ///
    /// Update Speed: 500ms
    pub fn symbol_ticker(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSFuturesTicker>> {
        self.build_single(symbol, "ticker")
    }

    /// 按Symbol刷新的24小时完整ticker信息
    ///
    /// Update Speed: 500ms
    pub fn symbol_ticker_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSFuturesTicker>> {
        self.build_multi(symbols, "ticker")
    }

    /// 全市场的完整Ticker
    ///
    /// 所有symbol 24小时完整ticker信息.需要注意的是，只有发生变化的ticker更新才会被推送。
    /// Update Speed: 1000ms
    pub fn all_symbol_ticker(
        &self,
    ) -> BianResult<impl WebsocketResponse<Vec<response::WSFuturesTicker>>> {
        self.build_single(String::new(), "!ticker@arr")
    }

    /// 按Symbol的最优挂单信息
    ///
    /// 实时推送指定交易对最优挂单信息 Update Speed: 实时
    pub fn book_ticker(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSFuturesBookTicker>> {
        self.build_single(symbol, "bookTicker")
    }

    /// 按Symbol的最优挂单信息
    ///
    /// 实时推送指定交易对最优挂单信息 Update Speed: 实时
    pub fn book_ticker_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSFuturesBookTicker>> {
        self.build_multi(symbols, "bookTicker")
    }

    /// 全市场最优挂单信息
    ///
    ///所有交易对交易对最优挂单信息
    pub fn all_book_ticker(
        &self,
    ) -> BianResult<impl WebsocketResponse<response::WSFuturesBookTicker>> {
        self.build_single(String::new(), "!bookTicker")
    }

    /// 强平订单
    ///
    /// 推送特定symbol的强平订单信息 Update Speed: 实时
    pub fn force_order(
        &self,
        symbol: String,
    ) -> BianResult<impl WebsocketResponse<response::WSForceOrder>> {
        self.build_single(symbol, "forceOrder")
    }

    /// 强平订单
    ///
    /// 推送特定symbol的强平订单信息 Update Speed: 实时
    pub fn force_order_multi(
        &self,
        symbols: Vec<String>,
    ) -> BianResult<impl WebsocketResponse<response::WSForceOrder>> {
        self.build_multi(symbols, "forceOrder")
    }

    /// 全市场强平订单
    ///
    /// 推送全市场强平订单信息 Update Speed: 实时
    pub fn all_force_order(&self) -> BianResult<impl WebsocketResponse<response::WSForceOrder>> {
        self.build_single(String::new(), "!forceOrder@arr")
    }

    /// 有限档深度信息
    ///
    /// 推送有限档深度信息。levels表示几档买卖单信息, 可选 5/10/20档
    /// Update Speed: 250ms 或 500ms 或 100ms
    pub fn limit_depth(
        &self,
        symbol: String,
        level: usize,
        freq: usize,
    ) -> BianResult<impl WebsocketResponse<response::WSFuturesDepth>> {
        let level = match level {
            10 => 10,
            20 => 20,
            _ => 5,
        };
        let channel = match freq {
            100 => format!("depth{}@100ms", level),
            500 => format!("depth{}@500ms", level),
            _ => format!("depth{}", level),
        };
        self.build_single(symbol, &channel)
    }

    /// 有限档深度信息
    ///
    /// 推送有限档深度信息。levels表示几档买卖单信息, 可选 5/10/20档
    /// Update Speed: 250ms 或 500ms 或 100ms
    pub fn limit_depth_multi(
        &self,
        symbols: Vec<String>,
        level: usize,
        freq: usize,
    ) -> BianResult<impl WebsocketResponse<response::WSFuturesDepth>> {
        let level = match level {
            10 => 10,
            20 => 20,
            _ => 5,
        };
        let channel = match freq {
            100 => format!("depth{}@100ms", level),
            500 => format!("depth{}@500ms", level),
            _ => format!("depth{}", level),
        };
        self.build_multi(symbols, &channel)
    }
}

/// 用户 data stream
impl UFuturesWSClient {
    pub fn user_data(
        &self,
        listen_key: &str,
    ) -> BianResult<impl WebsocketResponse<response::WSUserStream>> {
        let url = self.base_url.join(listen_key).unwrap();
        let (socket, _) = connect_with_config(url, None, 3, self.proxy)
            .map_err(|e| APIError::WSConnectError(e.to_string()))?;
        Ok(socket)
    }
}
