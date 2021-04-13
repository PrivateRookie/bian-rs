use std::usize;

use super::{string_as_f64, string_as_usize};
use crate::enums::{
    ContractType, FuturesOrderType, Interval, MarginType, OrderSide, OrderStatus, PositionDirect,
    TimeInForce,
};
use crate::error::{APIError, BianResult};
use serde::Deserialize;
use tungstenite::client::AutoGenericStream;

pub trait WebsocketResponse<R: serde::de::DeserializeOwned> {
    fn read_stream_single(&mut self) -> BianResult<R>;
    fn read_stream_multi(&mut self) -> BianResult<R>;
    fn close_stream(&mut self);
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
pub struct WSTrade {
    /// 事件类型 trade
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// 成交价格
    #[serde(rename = "p", deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 成交笔数
    #[serde(rename = "q", deserialize_with = "string_as_f64")]
    pub qty: f64,
    /// 买房的订单ID
    #[serde(rename = "b")]
    pub buyer_order_id: usize,
    /// 卖房的订单ID
    #[serde(rename = "b")]
    pub seller_order_id: usize,
    /// 成交时间
    #[serde(rename = "T")]
    pub trade_time: i64,
    /// 买方是否是做市方。如true，则此次成交是一个主动卖出单，否则是一个主动买入单。
    #[serde(rename = "m")]
    pub is_market: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSPrice {
    /// 事件类型 price
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// 标记价格
    #[serde(rename = "p", deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 现货指数价格
    #[serde(rename = "i", deserialize_with = "string_as_f64")]
    pub index_price: f64,
    /// 预估结算价，尽在结算前最后一小时有参考价值
    #[serde(rename = "P", deserialize_with = "string_as_f64")]
    pub estimate_price: f64,
    /// 资金费率
    #[serde(rename = "r", deserialize_with = "string_as_f64")]
    pub rate: f64,
    /// 下次资金时间
    #[serde(rename = "T")]
    pub trade_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSKline {
    /// 事件类型 kline
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// K 线数据
    #[serde(rename = "k")]
    pub data: KData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSContinuousKline {
    /// 事件类型 continuous_kline
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易对
    #[serde(rename = "ps")]
    pub pair: String,
    /// 合约类型
    #[serde(rename = "ct")]
    pub contract_type: ContractType,
    /// K 线数据
    #[serde(rename = "k")]
    pub data: KData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KData {
    /// 这根K线的起始时间
    #[serde(rename = "t")]
    pub open_time: i64,
    /// 这根K线的结束时间
    #[serde(rename = "T")]
    pub close_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// K线间隔
    #[serde(rename = "i")]
    pub interval: Interval,
    /// 这根K线期间第一笔成交ID
    #[serde(rename = "f")]
    pub first_id: i64,
    /// 这根K线期间末一笔成交ID
    #[serde(rename = "L")]
    pub last_id: i64,
    /// 这根K线期间第一笔成交价
    #[serde(rename = "o", deserialize_with = "string_as_f64")]
    pub first_price: f64,
    /// 这根K线期间末一笔成交价
    #[serde(rename = "c", deserialize_with = "string_as_f64")]
    pub last_price: f64,
    /// 这根K线期间最高成交价
    #[serde(rename = "h", deserialize_with = "string_as_f64")]
    pub high: f64,
    /// 这根K线期间最低成交价
    #[serde(rename = "l", deserialize_with = "string_as_f64")]
    pub low: f64,
    /// 这根K线期间成交量
    #[serde(rename = "v", deserialize_with = "string_as_f64")]
    pub volume: f64,
    /// 这根K线期间成交笔数
    #[serde(rename = "n")]
    pub trade_num: usize,
    /// 这根K线是否完结(是否已经开始下一根K线)
    #[serde(rename = "x")]
    pub is_end: bool,
    /// 这根K线期间成交额
    #[serde(rename = "q", deserialize_with = "string_as_f64")]
    pub qty: f64,
    /// 主动买入的成交量
    #[serde(rename = "V", deserialize_with = "string_as_f64")]
    pub take_volume: f64,
    /// 主动买入的成交额
    #[serde(rename = "Q", deserialize_with = "string_as_f64")]
    pub take_qty: f64,
    /// 忽略此参数
    #[serde(rename = "B")]
    pub __ignore: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSMiniTicker {
    /// 事件类型 24hrMiniTicker
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件时间(ms)
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub pair: String,
    /// 最新成交价格
    #[serde(rename = "c", deserialize_with = "string_as_f64")]
    pub price_last_trade: f64,
    /// 24小时前开始第一笔成交价格
    #[serde(rename = "o", deserialize_with = "string_as_f64")]
    pub price_24h_first_trade: f64,
    /// 24小时内最高成交价
    #[serde(rename = "h", deserialize_with = "string_as_f64")]
    pub high: f64,
    /// 24小时内最低成交价
    #[serde(rename = "l", deserialize_with = "string_as_f64")]
    pub low: f64,
    /// 成交量
    #[serde(rename = "v", deserialize_with = "string_as_f64")]
    pub volume: f64,
    /// 成交额
    #[serde(rename = "q", deserialize_with = "string_as_f64")]
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSTicker {
    /// 事件类型 24hrTicker
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub pair: String,
    /// 24小时价格变化
    #[serde(rename = "p", deserialize_with = "string_as_f64")]
    pub price_24h_chg: f64,
    /// 24小时价格变化(百分比)
    #[serde(rename = "P", deserialize_with = "string_as_f64")]
    pub price_24h_chg_pct: f64,
    /// 平均价格
    #[serde(rename = "w", deserialize_with = "string_as_f64")]
    pub price_avg: f64,
    /// 最新成交价格
    #[serde(rename = "c", deserialize_with = "string_as_f64")]
    pub price_last_trade: f64,
    /// 最新成交价格上的成交量
    #[serde(rename = "Q", deserialize_with = "string_as_f64")]
    pub volume_last_trade: f64,
    /// 24小时内第一笔成交的价格
    #[serde(rename = "o", deserialize_with = "string_as_f64")]
    pub price_24h_first_trade: f64,
    /// 24小时内最高成交价
    #[serde(rename = "h", deserialize_with = "string_as_f64")]
    pub high: f64,
    /// 24小时内最低成交价
    #[serde(rename = "l", deserialize_with = "string_as_f64")]
    pub low: f64,
    /// 24小时内成交量
    #[serde(rename = "v", deserialize_with = "string_as_f64")]
    pub volume: f64,
    /// 24小时内成交额
    #[serde(rename = "q", deserialize_with = "string_as_f64")]
    pub amount: f64,
    #[serde(rename = "O")]
    pub open_time: i64,
    #[serde(rename = "C")]
    pub close_time: i64,
    /// 24小时内第一笔成交交易ID
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    /// 24小时内最后一笔成交交易ID
    #[serde(rename = "L")]
    pub last_trade_id: f64,
    #[serde(rename = "n")]
    pub trade_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSBookTicker {
    /// 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 更新ID
    #[serde(rename = "u")]
    pub update_id: usize,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 撮合时间
    #[serde(rename = "T")]
    pub matching_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// 买单最优挂单价格
    #[serde(rename = "b", deserialize_with = "string_as_f64")]
    pub buy_price: f64,
    /// 买单最优挂单价格
    #[serde(rename = "B", deserialize_with = "string_as_f64")]
    pub buy_amount: f64,
    /// 买单最优挂单价格
    #[serde(rename = "a", deserialize_with = "string_as_f64")]
    pub sell_price: f64,
    /// 买单最优挂单价格
    #[serde(rename = "A", deserialize_with = "string_as_f64")]
    pub sell_amount: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSForceOrder {
    // 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "o")]
    pub order_data: OrderData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// 订单方向
    #[serde(rename = "S")]
    pub direction: OrderSide,
    /// 订单类型
    #[serde(rename = "o")]
    pub order_type: FuturesOrderType,
    /// 有效方式
    #[serde(rename = "f")]
    pub force: TimeInForce,
    /// 订单数量
    #[serde(rename = "q", deserialize_with = "string_as_f64")]
    pub qty: f64,
    /// 订单价格
    #[serde(rename = "p", deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 平均价格
    #[serde(rename = "ap", deserialize_with = "string_as_f64")]
    pub avg_price: f64,
    /// 订单状态
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    /// 订单最近成交量
    #[serde(rename = "l", deserialize_with = "string_as_f64")]
    pub latest_amount: f64,
    /// 订单累计成交量
    #[serde(rename = "z", deserialize_with = "string_as_f64")]
    pub sum_amount: f64,
    /// 交易时间
    #[serde(rename = "T")]
    pub trade_time: i64,
}

/// 有限档深度信息
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSDepth {
    /// 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 交易时间
    #[serde(rename = "T")]
    pub trade_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// 更新ID
    #[serde(rename = "u")]
    pub update_id: usize,
    /// ???
    #[serde(rename = "U")]
    pub upper_u: usize,
    /// ???
    pub pu: usize,
    /// 买方
    #[serde(rename = "b")]
    pub buy: Vec<(String, String)>,
    /// 卖方
    #[serde(rename = "a")]
    pub sell: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSListenKeyExpired {
    /// 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSMarginPosition {
    /// symbol
    #[serde(rename = "s")]
    pub symbol: String,
    /// 持仓方向
    #[serde(rename = "ps")]
    pub position_side: PositionDirect,
    /// 仓位
    #[serde(deserialize_with = "string_as_f64")]
    pub pa: f64,
    /// 保证金模式
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    /// 若为逐仓，仓位保证金
    #[serde(deserialize_with = "string_as_f64")]
    pub iw: f64,
    /// 标记价格
    #[serde(rename = "mp", deserialize_with = "string_as_f64")]
    pub mark_price: f64,
    /// 未实现盈亏
    #[serde(rename = "up", deserialize_with = "string_as_f64")]
    pub unrealized_pnl: f64,
    /// 持仓需要的维持保证金
    #[serde(rename = "mm", deserialize_with = "string_as_f64")]
    pub maint_margin: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSMarginCall {
    /// 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 除去逐仓仓位保证金的钱包余额, 仅在全仓 margin call 情况下推送此字段
    #[serde(deserialize_with = "string_as_f64")]
    pub cw: f64,
    /// 仓位数据
    #[serde(rename = "p")]
    pub position: Vec<WSMarginPosition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSAccountEvent {
    // TODO make it enum
    /// 事件推出原因
    #[serde(rename = "m")]
    pub update_type: String,
    #[serde(rename = "B")]
    pub balances: Vec<WSBalance>,
    #[serde(rename = "P")]
    pub positions: Vec<WSPosition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSBalance {
    /// 资产名称
    #[serde(rename = "a")]
    pub asset: String,
    /// 钱包余额
    #[serde(deserialize_with = "string_as_f64")]
    pub wb: f64,
    /// 除去逐仓仓位保证金的钱包余额
    #[serde(deserialize_with = "string_as_f64")]
    pub cw: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSPosition {
    /// 交易对
    #[serde(rename = "s")]
    pub symbol: String,
    /// 仓位
    #[serde(rename = "pa", deserialize_with = "string_as_f64")]
    pub position: f64,
    /// 入仓价格
    #[serde(rename = "ep", deserialize_with = "string_as_f64")]
    pub enter_price: f64,
    /// (费前)累计实现损益
    #[serde(rename = "cr", deserialize_with = "string_as_f64")]
    pub cumulative_realized: f64,
    /// 未实现盈亏
    #[serde(rename = "up", deserialize_with = "string_as_f64")]
    pub unrealized_pnl: f64,
    /// 保证金模式
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    /// 若为逐仓，仓位保证金
    #[serde(deserialize_with = "string_as_f64")]
    pub iw: f64,
    /// 持仓方向
    #[serde(rename = "ps")]
    pub position_side: PositionDirect,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSAccountUpdate {
    /// 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 撮合时间
    #[serde(rename = "T")]
    pub trade_matching_time: i64,
    /// 账户更新事件
    #[serde(rename = "a")]
    pub event: WSAccountEvent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSOrderUpdate {
    /// 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 撮合时间
    #[serde(rename = "T")]
    pub trade_matching_time: i64,
    /// 账户更新事件
    #[serde(rename = "a")]
    pub event: WSOrderEvent,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSOrderEvent {
    /// 交易对
    pub s: String,
    /// 客户端自定订单ID
    ///
    /// 特殊的自定义订单ID:
    /// - "autoclose-"开头的字符串: 系统强平订单
    /// - "adl_autoclose": ADL自动减仓订单
    pub c: String,
    /// 订单方向
    pub S: OrderSide,
    /// 订单类型
    pub o: FuturesOrderType,
    /// 有效方式
    pub f: TimeInForce,
    /// 订单原始数量
    #[serde(deserialize_with = "string_as_f64")]
    pub q: f64,
    /// 订单原始价格
    #[serde(deserialize_with = "string_as_f64")]
    pub p: f64,
    /// 订单平均价格
    #[serde(deserialize_with = "string_as_f64")]
    pub ap: f64,
    /// 条件订单触发价格，对追踪止损单无效
    #[serde(deserialize_with = "string_as_f64")]
    pub sp: f64,
    /// 本次事件的具体执行类型
    pub x: String,
    /// 订单的当前状态
    pub X: OrderStatus,
    /// 订单ID
    pub i: usize,
    /// 订单末次成交量
    #[serde(deserialize_with = "string_as_f64")]
    pub l: f64,
    /// 订单累计已成交量
    #[serde(deserialize_with = "string_as_f64")]
    pub z: f64,
    /// 订单末次成交价格
    #[serde(deserialize_with = "string_as_f64")]
    pub L: f64,
    /// 手续费资产类型
    pub N: String,
    /// 手续费数量
    #[serde(deserialize_with = "string_as_f64")]
    pub n: f64,
    /// 成交时间
    pub T: i64,
    /// 成交ID
    pub t: usize,
    /// 买单净值
    #[serde(deserialize_with = "string_as_f64")]
    pub b: f64,
    /// 卖单净值
    #[serde(deserialize_with = "string_as_f64")]
    pub a: f64,
    /// 该成交是作为挂单成交吗？
    pub m: bool,
    /// 是否是只减仓单
    pub R: bool,
    /// 触发价类型
    pub wt: String,
    /// 原始订单类型
    pub ot: FuturesOrderType,
    /// 持仓方向
    pub ps: PositionDirect,
    /// 是否为触发平仓单; 仅在条件订单情况下会推送此字段
    pub cp: bool,
    /// 追踪止损激活价格, 仅在追踪止损单时会推送此字段
    #[serde(deserialize_with = "string_as_f64")]
    pub AP: f64,
    /// 追踪止损回调比例, 仅在追踪止损单时会推送此字段
    #[serde(deserialize_with = "string_as_f64")]
    pub cr: f64,
    /// 该交易实现盈亏
    #[serde(deserialize_with = "string_as_f64")]
    pub rp: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSLeverageUpdate {
    /// 事件类型
    #[serde(rename = "e")]
    pub event_type: String,
    /// 事件推送时间
    #[serde(rename = "E")]
    pub event_time: i64,
    /// 撮合时间
    #[serde(rename = "T")]
    pub trade_matching_time: i64,
    #[serde(rename = "ac")]
    pub event: WSLeverageEvent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSLeverageEvent {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l", deserialize_with = "string_as_usize")]
    pub leverage: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WSUserStream {
    ListenKeyExpired(WSListenKeyExpired),
    MarginPosition(WSMarginPosition),
    AccountUpdate(WSAccountUpdate),
    OrderUpdate(WSOrderUpdate),
    LeverageUpdate(WSLeverageUpdate),
}

impl<R: serde::de::DeserializeOwned> WebsocketResponse<R>
    for tungstenite::WebSocket<AutoGenericStream>
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

    fn close_stream(&mut self) {
        self.close(None).unwrap();
    }
}
