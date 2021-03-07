use std::{collections::HashMap, fmt};

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer,
};

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
    // TODO to enum
    pub order_types: Vec<String>,
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
