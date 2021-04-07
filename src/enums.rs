use serde::{Deserialize, Serialize};

/// 合约类型
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    /// 永续合约
    Perpetual,
    /// 当月交割合约
    CurrentMonth,
    /// 次月交割合约
    NextMonth,
    /// 交割中的无效类型
    CurrentMonthDelivering,
    /// 交割中的无效类型
    NextMonthDelivering,
}

impl ToString for ContractType {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .unwrap()
            .trim_matches('"')
            .to_string()
    }
}

/// 合约状态
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    ///  待上市
    PendingTrading,
    /// 交易中
    Trading,
    /// 预交割
    PreDelivering,
    /// 交割中
    Delivering,
    /// 已交割
    Delivered,
    /// 预结算
    PreSettle,
    /// 结算中
    Settling,
    /// 已下架
    Close,
}

/// 订单状态
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// 新建订单
    New,
    /// 部分成交
    PartiallyFilled,
    /// 全部成交
    Filled,
    /// 已撤销
    Canceled,
    /// 订单被拒绝
    Rejected,
    /// 订单过期(根据timeInForce参数规则)
    Expired,
}

/// 期货订单种类
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FuturesOrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

/// 现货订单种类
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpotOrderType {
    ///限价单
    Limit,
    ///市价单
    Market,
    ///止损单
    StopLoss,
    ///限价止损单
    StopLossLimit,
    ///止盈单
    TakeProfit,
    ///限价止盈单
    TakeProfitLimit,
    ///限价只挂单
    LimitMaker,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionDirect {
    Both,
    Long,
    Short,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginType {
    Isolated,
    Crossed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NewOrderType {
    Ack,
    Result,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Interval {
    #[serde(rename = "1m")]
    Min1,
    #[serde(rename = "3m")]
    Min3,
    #[serde(rename = "5m")]
    Min5,
    #[serde(rename = "15m")]
    Min15,
    #[serde(rename = "30m")]
    Min30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "2h")]
    Hour2,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "6h")]
    Hour6,
    #[serde(rename = "8h")]
    Hour8,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "3d")]
    Day3,
    #[serde(rename = "1w")]
    Week1,
    #[serde(rename = "1M")]
    Month1,
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .unwrap()
            .trim_matches('"')
            .to_string()
    }
}

pub enum RateLimitType {
    RequestsWeight,
    Orders,
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    SGET,
    SPOST,
    SPUT,
    SDELETE,
}
