use std::usize;

use crate::enums::Interval;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PDepth {
    pub symbol: String,
    /// 默认 500; 可选值:[5, 10, 20, 50, 100, 500, 1000]
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
pub struct PPremiumIndex {
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
pub struct AccountBalanceV2 {
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}
