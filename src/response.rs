use std::{collections::HashMap, fmt, usize};

use crate::enums::{OrderSide, OrderStatus, OrderType, TimeInForce};
use crate::error::{APIError, BianResult};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer,
};
use tungstenite::client::ProxyAutoStream;

struct F64Visitor;

impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a f64")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<f64>()
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &"a string representation as f64"))
    }
}

fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(F64Visitor)
}

struct UsizeVisitor;

impl<'de> Visitor<'de> for UsizeVisitor {
    type Value = usize;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a usize")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<usize>()
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &"a string representation as usize"))
    }
}

fn string_as_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(UsizeVisitor)
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ping(HashMap<String, String>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_alias: String,
    pub asset: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub balance: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub cross_wallet_balance: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub cross_un_pnl: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub available_balance: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub max_withdraw_amount: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    /// 按照分钟计算
    pub interval: String,
    /// 按照1分钟计算
    pub interval_num: usize,
    /// 上限次数
    pub limit: usize,
    /// 按照访问权重来计算
    pub rate_limit_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "filterType")]
pub enum SymbolFilter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(deserialize_with = "string_as_f64", rename = "minPrice")]
        min_price: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "maxPrice")]
        max_price: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "tickSize")]
        tick_size: f64,
    },
    #[serde(rename = "LOT_SIZE")]
    LOTSize {
        #[serde(deserialize_with = "string_as_f64", rename = "stepSize")]
        step_size: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "maxQty")]
        max_qty: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "minQty")]
        min_qty: f64,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLOTSize {
        #[serde(deserialize_with = "string_as_f64", rename = "stepSize")]
        step_size: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "maxQty")]
        max_qty: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "minQty")]
        min_qty: f64,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders { limit: usize },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders { limit: usize },
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional {
        #[serde(deserialize_with = "string_as_usize")]
        notional: usize,
    },
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice {
        #[serde(deserialize_with = "string_as_f64", rename = "multiplierDown")]
        multiplier_down: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "multiplierUp")]
        multiplier_up: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "multiplierDecimal")]
        multiplier_decimal: f64,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// 交易对
    pub symbol: String,
    /// 标的交易对
    pub pair: String,
    /// 合约类型
    pub contract_type: String,
    /// 交割日期
    pub delivery_date: i64,
    /// 上线日期
    pub onboard_date: i64,
    /// 交易对状态
    pub status: String,
    #[serde(deserialize_with = "string_as_f64")]
    /// 请忽略
    pub maint_margin_percent: f64,
    #[serde(deserialize_with = "string_as_f64")]
    /// 请忽略
    pub required_margin_percent: f64,
    /// 标的资产
    pub base_asset: String,
    /// 报价资产
    pub quote_asset: String,
    /// 保证金资产
    pub margin_asset: String,
    /// 价格小数点位数
    pub price_precision: usize,
    /// 数量小数点位数
    pub quantity_precision: usize,
    /// 标的资产精度
    pub base_asset_precision: usize,
    /// 报价资产精度
    pub quote_precision: usize,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: usize,
    #[serde(deserialize_with = "string_as_f64")]
    /// 开启"priceProtect"的条件订单的触发阈值
    pub trigger_protect: f64,
    pub filters: Vec<SymbolFilter>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub exchange_filters: Vec<String>,
    pub rate_limits: Vec<RateLimit>,
    pub futures_type: String,
    pub server_time: i64,
    pub symbols: Vec<Symbol>,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Depth {
    pub last_update_id: usize,
    /// 消息时间
    #[serde(rename = "E")]
    pub e: usize,
    /// 撮合引擎时间
    #[serde(rename = "T")]
    pub t: usize,
    // TODO convert string to float
    /// 买单
    pub bids: Vec<(String, String)>,
    /// 卖单
    pub asks: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: usize,
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub quote_qty: f64,
    pub time: i64,
    pub is_buyer_maker: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTrade {
    pub id: usize,
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub quote_qty: f64,
    pub time: i64,
    pub is_buyer_maker: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTrade {
    pub a: i64,
    #[serde(deserialize_with = "string_as_f64")]
    pub p: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub q: f64,
    pub f: i64,
    pub l: i64,
    #[serde(rename = "T")]
    pub t: i64,
    pub m: bool,
}

// TODO make it to map
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    usize,
    String,
    String,
    String,
);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndex {
    symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    mark_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub index_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub last_funding_rate: f64,
    pub next_funding_time: i64,
    #[serde(deserialize_with = "string_as_f64")]
    pub interest_rate: f64,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub funding_rate: f64,
    pub funding_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct H24ticker {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub price_change: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub price_change_percent: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub weighted_avg_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub last_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub last_qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub open_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub high_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub low_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub volume: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub quote_volume: f64,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: usize,
    pub last_id: usize,
    pub count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub bid_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub bid_qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub ask_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub ask_qty: f64,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceOrder {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub orig_qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub executed_qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub average_price: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// 未平仓合约数量
    #[serde(deserialize_with = "string_as_f64")]
    pub open_interest: f64,
    pub symbol: String,
    /// 撮合引擎时间
    pub time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHist {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub sum_open_interest: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub sum_open_interest_value: f64,
    #[serde(deserialize_with = "string_as_usize")]
    pub timestamp: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub long_short_ratio: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub long_account: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub short_account: f64,
    #[serde(deserialize_with = "string_as_usize")]
    pub timestamp: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerLongShortRatio {
    #[serde(deserialize_with = "string_as_f64")]
    pub buy_sell_ratio: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub buy_vol: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub sell_vol: f64,
    #[serde(deserialize_with = "string_as_usize")]
    pub timestamp: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexInfo {
    pub symbol: String,
    pub time: i64,
    pub base_asset_list: Vec<BaseAsset>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAsset {
    pub base_asset: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub weight_in_quantity: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub weight_in_percentage: f64,
}

pub trait WebsocketResponse<R: serde::de::DeserializeOwned> {
    fn read_stream_single(&mut self) -> BianResult<R>;
    fn read_stream_multi(&mut self) -> BianResult<R>;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiResponse<R> {
    pub stream: String,
    pub data: R,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSAggTrade {
    /// 事件类型 aggTrade
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// 归集成交 ID
    #[serde(rename = "a")]
    pub agg_id: usize,
    /// 成交价格
    #[serde(rename = "p", deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 成交量
    #[serde(rename = "q", deserialize_with = "string_as_f64")]
    pub qty: f64,
    /// 被归集的首个交易ID
    #[serde(rename = "f")]
    pub first_trade_id: usize,
    /// 被归集的末次交易ID
    #[serde(rename = "l")]
    pub last: usize,
    /// 成交时间
    #[serde(rename = "T")]
    pub trade_time: i64,
    /// 买方是否是做市方。如true，则此次成交是一个主动卖出单，否则是一个主动买入单。
    #[serde(rename = "m")]
    pub is_market: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub pair: String,
    #[serde(rename = "p", deserialize_with = "string_as_f64")]
    pub price_24h_chg: f64,
    #[serde(rename = "P", deserialize_with = "string_as_f64")]
    pub price_24h_chg_pct: f64,
    #[serde(rename = "w", deserialize_with = "string_as_f64")]
    pub price_avg: f64,
    #[serde(rename = "c", deserialize_with = "string_as_f64")]
    pub price_last_trade: f64,
    #[serde(rename = "Q", deserialize_with = "string_as_f64")]
    pub volume_last_trade: f64,
    #[serde(rename = "o", deserialize_with = "string_as_f64")]
    pub price_24h_first_trade: f64,
    #[serde(rename = "h", deserialize_with = "string_as_f64")]
    pub price_24h_highest: f64,
    #[serde(rename = "l", deserialize_with = "string_as_f64")]
    pub price_24h_lowest: f64,
    #[serde(rename = "v", deserialize_with = "string_as_f64")]
    pub volume_24h: f64,
    #[serde(rename = "q", deserialize_with = "string_as_f64")]
    pub amount_24h: f64,
    #[serde(rename = "O")]
    pub open_time: i64,
    #[serde(rename = "C")]
    pub close_time: i64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: f64,
    #[serde(rename = "n")]
    pub trade_count: usize,
}

impl<R: serde::de::DeserializeOwned> WebsocketResponse<R>
    for tungstenite::WebSocket<ProxyAutoStream>
{
    fn read_stream_single(&mut self) -> BianResult<R> {
        let msg = self
            .read_message()
            .map_err(|e| APIError::WSClientError(e.to_string()))?;
        match msg {
            tungstenite::Message::Text(text) => {
                let resp = serde_json::from_str(&text)
                    .map_err(|e| APIError::DecodeError(e.to_string()))?;
                Ok(resp)
            }
            tungstenite::Message::Ping(_) => {
                let pong = tungstenite::Message::Pong(vec![]);
                self.write_message(pong)
                    .map_err(|e| APIError::WSClientError(e.to_string()))?;
                self.read_stream_single()
            }
            _ => unreachable!(),
        }
    }

    fn read_stream_multi(&mut self) -> BianResult<R> {
        let msg = self
            .read_message()
            .map_err(|e| APIError::WSClientError(e.to_string()))?;
        match msg {
            tungstenite::Message::Text(text) => {
                let wrapped_resp: MultiResponse<R> = serde_json::from_str(&text)
                    .map_err(|e| APIError::DecodeError(e.to_string()))?;
                Ok(wrapped_resp.data)
            }
            tungstenite::Message::Ping(_) => {
                let pong = tungstenite::Message::Pong(vec![]);
                self.write_message(pong)
                    .map_err(|e| APIError::WSClientError(e.to_string()))?;
                self.read_stream_multi()
            }
            _ => unreachable!(),
        }
    }
}
