use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Ping(HashMap<String, String>);

#[derive(Debug, Serialize)]
pub struct AccountBalanceV2 {
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}
