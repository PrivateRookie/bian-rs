pub enum ContractType {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivered,
    PreSettle,
    Settling,
    Close,
}

pub enum OrderStatus {
    New,
    PartialFilled,
    Canceled,
    Rejected,
    Expired,
}

pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TailingStopMarket,
}

pub enum OrderSide {
    Buy,
    Sell,
}

pub enum PositionDirect {
    Both,
    Long,
    Short,
}

pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
}

pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

pub enum NewOrderType {
    Ack,
    Result,
}

pub enum Interval {
    Min1,
    Min3,
    Min5,
    Min15,
    Min30,
    Hour1,
    Hour2,
    Hour4,
    Hour6,
    Hour8,
    Hour12,
    Day1,
    Day3,
    Week1,
    Month1,
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
