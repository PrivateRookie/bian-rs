use std::usize;

use crate::enums::{Interval, MarginType, OrderSide, OrderType, PositionDirect, TimeInForce};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PDepth {
    pub symbol: String,
    pub limit: usize,
}

#[derive(Debug, Serialize)]
pub struct PTrade {
    pub symbol: String,
    /// 默认 500; 默认:500，最大1000
    pub limit: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PHistoricalTrade {
    pub symbol: String,
    /// 默认值:500 最大值:1000.
    pub limit: Option<usize>,
    pub from_id: Option<usize>,
}

/// 如果同时发送startTime和endTime，间隔必须小于一小时
/// 如果没有发送任何筛选参数(fromId, startTime, endTime)，默认返回最近的成交记录
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PAggTrade {
    pub symbol: String,
    /// 默认值:500 最大值:1000.
    pub limit: Option<usize>,
    pub from_id: Option<usize>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PKline {
    pub symbol: String,
    pub interval: Interval,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PContinuousKline {
    pub pair: String,
    pub contract_type: String,
    pub interval: Interval,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    /// 默认值:500 最大值:1500
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PIndexPriceKline {
    pub pair: String,
    pub interval: Interval,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct PSymbol {
    pub symbol: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PFundingRate {
    pub symbol: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    /// 默认值:100 最大值:1000
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PForceOrder {
    pub symbol: Option<String>,
    pub start_time: Option<i64>,
    /// 结束时间,默认当前时间
    pub end_time: Option<i64>,
    /// 从endTime倒推算起的数据条数，默认值:100 最大值:1000
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PFutures {
    pub symbol: String,
    pub period: Interval,
    /// 默认30，最大500
    pub limit: Option<usize>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PLvtKlines {
    pub symbol: String,
    pub interval: Interval,
    /// 默认30，最大500
    pub limit: Option<usize>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PTimestamp {
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

impl PTimestamp {
    pub fn now() -> Self {
        let now = chrono::Utc::now();
        PTimestamp {
            timestamp: now.timestamp_millis(),
            recv_window: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PPositionSideDual {
    /// "true": 双向持仓模式；"false": 单向持仓模式
    pub dual_side_position: bool,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct POrderSpec {
    /// 交易对
    pub symbol: String,
    /// 买卖方向 `SELL`, `BUY`
    pub side: OrderSide,
    /// 持仓方向，单向持仓模式下非必填，默认且仅可填`BOTH`;在双向持仓模式下必填,且仅可选择 `LONG` 或 `SHORT`
    pub position_side: Option<PositionDirect>,
    /// 订单类型 `LIMIT`, `MARKET`, `STOP`, `TAKE_PROFIT`, `STOP_MARKET`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// `true`, `false`; 非双开模式下默认`false`；双开模式下不接受此参数； 使用`closePosition`不支持此参数。
    pub reduce_only: Option<bool>,
    /// 下单数量,使用`closePosition`不支持此参数。
    pub quantity: Option<f64>,
    /// 委托价格
    pub price: Option<f64>,
    /// 用户自定义的订单号，不可以重复出现在挂单中。如空缺系统会自动赋值。必须满足正则规则 `^[\.A-Z\:/a-z0-9_-]{1,36}$`
    pub new_client_order_id: Option<String>,
    /// 触发价, 仅 `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET` 需要此参数
    pub stop_price: Option<f64>,
    /// `true`, `false`；触发后全部平仓，仅支持`STOP_MARKET`和`TAKE_PROFIT_MARKET`；不与`quantity`合用；自带只平仓效果，不与`reduceOnly` 合用
    pub close_position: Option<f64>,
    /// 追踪止损激活价格，仅`TRAILING_STOP_MARKET` 需要此参数, 默认为下单当前市场价格(支持不同`workingType`)
    pub activation_price: Option<f64>,
    /// 追踪止损回调比例，可取值范围[0.1, 5],其中 1代表1% ,仅`TRAILING_STOP_MARKET` 需要此参数
    pub callback_rate: Option<f64>,
    /// 有效方法
    pub time_in_force: Option<TimeInForce>,
    // TODO make it enum
    /// stopPrice 触发类型: `MARK_PRICE`(标记价格), `CONTRACT_PRICE`(合约最新价). 默认 `CONTRACT_PRICE`
    pub working_type: Option<String>,
    // TODO make it enum
    /// 条件单触发保护："TRUE","FALSE", 默认"FALSE". 仅 `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET` 需要此参数
    pub price_protect: Option<String>,
    // TODO make it enum
    /// "ACK", "RESULT", 默认 "ACK"
    pub new_order_resp_type: Option<String>,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct POrder {
    #[serde(flatten)]
    pub spec: POrderSpec,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PBatchOrder {
    /// 订单列表，最多支持5个订单
    pub batch_orders: Vec<POrderSpec>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PQueryOrder {
    pub symbol: String,
    pub order_id: Option<usize>,
    pub orig_client_order_id: Option<String>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PSymbolWithTs {
    pub symbol: String,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PBatchCancelOrder {
    pub symbol: String,
    /// 系统订单号, 最多支持10个订单 比如\[1234567,2345678\]
    pub order_id_list: Vec<usize>,
    /// 用户自定义的订单号, 最多支持10个订单 比如["my_id_1","my_id_2"] 需要encode双引号。逗号后面没有空格。
    pub orig_client_order_id_list: Vec<String>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PCountdownCancel {
    pub symbol: String,
    pub countdown_time: i64,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct POptionSymbolQuery {
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PHistOrder {
    pub symbol: String,
    pub order_id: Option<usize>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    limit: Option<usize>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PLeverage {
    pub symbol: String,
    /// 目标杠杆倍数：1 到 125 整数
    pub leverage: usize,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PMarginType {
    pub symbol: String,
    pub margin_type: MarginType,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PPositionMargin {
    pub symbol: String,
    /// 持仓方向，单向持仓模式下非必填，默认且仅可填BOTH;在双向持仓模式下必填,且仅可选择 LONG 或 SHORT
    pub position_side: Option<PositionDirect>,
    pub amount: f64,
    /// 调整方向 1: 增加逐仓保证金，2: 减少逐仓保证金
    #[serde(rename = "type")]
    pub margin_type: usize,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PPositionMarginHist {
    pub symbol: String,
    /// 持仓方向，单向持仓模式下非必填，默认且仅可填BOTH;在双向持仓模式下必填,且仅可选择 LONG 或 SHORT
    pub position_side: Option<PositionDirect>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    /// 返回的结果集数量 默认值: 500
    pub limit: Option<usize>,
    #[serde(rename = "type")]
    pub margin_type: usize,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PUserTrade {
    pub symbol: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    /// 返回该fromId及之后的成交，缺省返回最近的成交
    pub from_id: Option<usize>,
    /// 返回的结果集数量 默认值:500 最大值:1000.
    pub limit: Option<usize>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}
