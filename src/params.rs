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
pub struct AccountBalanceV2 {
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}
