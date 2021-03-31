use std::usize;

use super::string_as_f64;
use crate::enums::{ContractType, Interval, OrderSide, OrderStatus, OrderType, TimeInForce};
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
pub struct WSPrice {
    /// 事件类型 aggTrade
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
    /// 事件类型 aggTrade
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
    /// 事件类型 aggTrade
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
    pub first_id: usize,
    /// 这根K线期间末一笔成交ID
    #[serde(rename = "L")]
    pub last_id: usize,
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
    /// 事件类型
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
    /// 事件类型
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
    pub order_type: OrderType,
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
