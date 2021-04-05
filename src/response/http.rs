use std::{collections::HashMap, usize};

use crate::enums::{MarginType, OrderSide, OrderStatus, OrderType, PositionDirect, TimeInForce};

use super::{string_as_f64, string_as_usize};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyResponse(HashMap<String, String>);

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
pub enum FuturesSymbolFilter {
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
#[serde(rename_all = "camelCase", tag = "filterType")]
pub enum SpotSymbolFilter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(deserialize_with = "string_as_f64", rename = "minPrice")]
        min_price: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "maxPrice")]
        max_price: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "tickSize")]
        tick_size: f64,
    },
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice {
        #[serde(deserialize_with = "string_as_f64", rename = "multiplierDown")]
        multiplier_down: f64,
        #[serde(deserialize_with = "string_as_f64", rename = "multiplierUp")]
        multiplier_up: f64,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: usize,
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
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional {
        #[serde(deserialize_with = "string_as_f64", rename = "minNotional")]
        min_notional: f64,
        #[serde(rename = "applyToMarket")]
        apply_to_market: bool,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: usize,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts { limit: usize },
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
    MaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max_number_orders: usize,
    },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: usize,
    },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders {
        #[serde(rename = "maxNumIcebergOrders")]
        max_num_iceberg_orders: usize,
    },
    #[serde(rename = "MAX_POSITION")]
    MaxPosition {
        #[serde(deserialize_with = "string_as_f64")]
        max_position: f64,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDERS")]
    ExchangeMaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max_num_orders: usize,
    },
    #[serde(rename = "EXCHANGE_MAX_ALGO_ORDERS")]
    ExchangeMaxAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: usize,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeResponse {
    pub code: usize,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesSymbol {
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
    pub filters: Vec<FuturesSymbolFilter>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotSymbol {
    /// 交易对
    pub symbol: String,
    /// 交易对状态
    pub status: String,
    /// 标的资产
    pub base_asset: String,
    /// 标的资产精度
    pub base_asset_precision: usize,
    pub quote_asset: String,
    pub quote_asset_precision: usize,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<SpotSymbolFilter>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesExchangeInfo {
    pub exchange_filters: Vec<String>,
    pub rate_limits: Vec<RateLimit>,
    pub futures_type: String,
    pub server_time: i64,
    pub symbols: Vec<FuturesSymbol>,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotExchangeInfo {
    pub exchange_filters: Vec<String>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: i64,
    pub symbols: Vec<SpotSymbol>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSide {
    /// "true": 双向持仓模式；"false": 单向持仓模式
    pub dual_side_position: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub client_order_id: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub cum_qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub cum_quote: f64,
    pub order_id: usize,
    #[serde(deserialize_with = "string_as_f64")]
    pub avg_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub orig_qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    #[serde(deserialize_with = "string_as_f64")]
    pub stop_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub close_position: f64,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(deserialize_with = "string_as_f64")]
    pub activate_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub price_rate: f64,
    pub update_time: i64,
    pub working_type: String,
    pub price_protect: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BatchOrderResponse {
    Order(Order),
    Code(CodeResponse),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancel {
    pub symbol: String,
    #[serde(deserialize_with = "string_as_usize")]
    pub countdown_time: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAsset {
    ///资产
    pub asset: String,
    ///余额
    #[serde(deserialize_with = "string_as_f64")]
    pub wallet_balance: f64,
    /// 未实现盈亏
    #[serde(deserialize_with = "string_as_f64")]
    pub unrealized_profit: f64,
    /// 保证金余额
    #[serde(deserialize_with = "string_as_f64")]
    pub margin_balance: f64,
    /// 维持保证金
    #[serde(deserialize_with = "string_as_f64")]
    pub maint_margin: f64,
    /// 当前所需起始保证金
    #[serde(deserialize_with = "string_as_f64")]
    pub initial_margin: f64,
    /// 持仓所需起始保证金(基于最新标记价格)
    #[serde(deserialize_with = "string_as_f64")]
    pub position_initial_margin: f64,
    /// 当前挂单所需起始保证金(基于最新标记价格)
    #[serde(deserialize_with = "string_as_f64")]
    pub open_order_initial_margin: f64,
    ///全仓账户余额
    #[serde(deserialize_with = "string_as_f64")]
    pub cross_wallet_balance: f64,
    /// 全仓持仓未实现盈亏
    #[serde(deserialize_with = "string_as_f64")]
    pub cross_un_pnl: f64,
    /// 可用余额
    #[serde(deserialize_with = "string_as_f64")]
    pub available_balance: f64,
    /// 最大可转出余额
    #[serde(deserialize_with = "string_as_f64")]
    pub max_withdraw_amount: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPosition {
    /// 交易对
    pub symbol: String,
    /// 当前所需起始保证金(基于最新标记价格)
    #[serde(deserialize_with = "string_as_f64")]
    pub initial_margin: f64,
    ///维持保证金
    #[serde(deserialize_with = "string_as_f64")]
    pub maint_margin: f64,
    /// 持仓未实现盈亏
    #[serde(deserialize_with = "string_as_f64")]
    pub unrealized_profit: f64,
    /// 持仓所需起始保证金(基于最新标记价格)
    #[serde(deserialize_with = "string_as_f64")]
    pub position_initial_margin: f64,
    /// 当前挂单所需起始保证金(基于最新标记价格)
    #[serde(deserialize_with = "string_as_f64")]
    pub open_order_initial_margin: f64,
    /// 杠杆倍率
    #[serde(deserialize_with = "string_as_f64")]
    pub leverage: f64,
    /// 是否是逐仓模式
    pub isolated: bool,
    /// 持仓成本价
    #[serde(deserialize_with = "string_as_f64")]
    pub entry_price: f64,
    /// 当前杠杆下用户可用的最大名义价值
    #[serde(deserialize_with = "string_as_f64")]
    pub max_notional: f64,
    /// 持仓方向
    pub position_side: PositionDirect,
    /// 持仓数量
    #[serde(deserialize_with = "string_as_f64")]
    pub position_amt: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// 手续费等级
    pub fee_tier: i64,
    /// 是否可以交易
    pub can_trade: bool,
    /// 是否可以入金
    pub can_deposit: bool,
    /// 是否可以出金
    pub can_withdraw: bool,
    pub update_time: i64,

    /// 但前所需起始保证金总额(存在逐仓请忽略), 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_initial_margin: f64,
    /// 维持保证金总额, 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_maint_margin: f64,
    /// 账户总余额, 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_wallet_balance: f64,
    /// 持仓未实现盈亏总额, 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_unrealized_profit: f64,
    /// 保证金总余额, 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_margin_balance: f64,
    /// 持仓所需起始保证金(基于最新标记价格), 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_position_initial_margin: f64,
    /// 当前挂单所需起始保证金(基于最新标记价格), 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_open_order_initial_margin: f64,
    /// 全仓账户余额, 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_cross_wallet_balance: f64,
    /// 全仓持仓未实现盈亏总额, 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub total_cross_un_pnl: f64,
    /// 可用余额, 仅计算usdt资产
    #[serde(deserialize_with = "string_as_f64")]
    pub available_balance: f64,
    /// 最大可转出余额, 仅计算usdt资产    
    #[serde(deserialize_with = "string_as_f64")]
    pub max_withdraw_amount: f64,
    /// 产品资产
    pub assets: Vec<AccountAsset>,
    /// 头寸，将返回所有市场symbol
    /// 根据用户持仓模式展示持仓方向，即双向模式下只返回BOTH持仓情况，单向模式下只返回 LONG 和 SHORT 持仓情况
    pub positions: Vec<AccountPosition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    pub symbol: String,
    pub leverage: usize,
    #[serde(deserialize_with = "string_as_f64")]
    pub max_notional_value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionMargin {
    pub amount: f64,
    pub code: usize,
    pub msg: String,
    /// 调整方向 1: 增加逐仓保证金，2: 减少逐仓保证金
    #[serde(rename = "type")]
    pub margin_type: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionMarginHist {
    #[serde(deserialize_with = "string_as_f64")]
    pub amount: f64,
    pub asset: String,
    pub symbol: String,
    pub time: i64,
    /// 调整方向 1: 增加逐仓保证金，2: 减少逐仓保证金
    #[serde(rename = "type")]
    pub margin_type: usize,
    pub position_side: PositionSide,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    // 开仓均价
    #[serde(deserialize_with = "string_as_f64")]
    pub entry_price: f64,
    // 逐仓模式或全仓模式
    pub margin_type: MarginType,

    pub is_auto_add_margin: bool,
    // 逐仓保证金
    #[serde(deserialize_with = "string_as_f64")]
    pub isolated_margin: f64,
    // 当前杠杆倍数
    #[serde(deserialize_with = "string_as_usize")]
    pub leverage: usize,
    // 参考强平价格
    #[serde(deserialize_with = "string_as_f64")]
    pub liquidation_price: f64,
    // 当前标记价格
    #[serde(deserialize_with = "string_as_f64")]
    pub mark_price: f64,
    // 当前杠杆倍数允许的名义价值上限
    #[serde(deserialize_with = "string_as_f64")]
    pub max_notional_value: f64,
    // 头寸数量，符号代表多空方向, 正数为多，负数为空
    #[serde(deserialize_with = "string_as_f64")]
    pub position_amt: f64,
    // 交易对
    pub symbol: String,
    // 持仓未实现盈亏
    #[serde(deserialize_with = "string_as_f64")]
    pub un_realized_profit: f64,
    // 持仓方向
    pub position_side: PositionSide,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTrade {
    /// 是否是买方
    buyer: bool,
    /// 手续费
    #[serde(deserialize_with = "string_as_f64")]
    commission: f64,
    /// 手续费计价单位
    commission_asset: String,
    /// 交易ID
    id: usize,
    /// 是否是挂单方
    maker: bool,
    /// 订单编号
    order_id: usize,
    /// 成交价
    #[serde(deserialize_with = "string_as_f64")]
    price: f64,
    /// 成交量
    #[serde(deserialize_with = "string_as_f64")]
    qty: f64,
    /// 成交额
    #[serde(deserialize_with = "string_as_f64")]
    quote_qty: f64,
    /// 实现盈亏
    #[serde(deserialize_with = "string_as_f64")]
    realized_pnl: f64,
    /// 买卖方向
    side: OrderSide,
    /// 持仓方向
    position_side: PositionSide,
    /// 交易对
    symbol: String,
    /// 时间
    time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenKey {
    pub listen_key: String,
}
