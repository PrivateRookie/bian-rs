use crate::error::{APIError, BianResult};
use crate::{params, response};
use bian_proc::api;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

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

    // /// 撤销单一交易对的所有挂单
    // ///
    // /// 撤销单一交易对下所有挂单, 包括OCO的挂单。
    // #[api(SDELETE "api/v3/openOrders")]
    // pub async fn cancel_open_orders(
    //     &self,
    //     param: params::PSymbolWithTs,
    // ) -> BianResult<Vec<response::SpotOpOrder>> {
    // }

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
