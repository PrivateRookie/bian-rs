use bian_rs::*;

use std::env;
const BASE_URL: &str = "https://fapi.binance.com/";

fn init_test() -> (String, String) {
    dotenv::dotenv().unwrap();
    let api_key = env::var("API_KEY").expect("can not find API_KEY env variable");
    let secret_key = env::var("SECRET_KEY").expect("can not find SECRET_KEY env variable");
    (api_key, secret_key)
}

#[tokio::test]
async fn test_ping() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    client.ping().await.unwrap();
}

#[tokio::test]
async fn test_balance() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    let now = chrono::Utc::now();
    let params = params::AccountBalanceV2 {
        timestamp: now.timestamp_millis(),
        recv_window: None,
    };
    client.account_balance_v2(params).await.unwrap();
}

#[tokio::test]
async fn test_server_time() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    client.server_time().await.unwrap();
}
#[tokio::test]
async fn test_exchange_info() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    client.exchange_info().await.unwrap();
}

#[tokio::test]
async fn test_depth() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    let param = params::PDepth {
        symbol: "BTCUSDT".to_string(),
        limit: 500,
    };
    dbg!(client.depth(param).await.unwrap());
}

#[tokio::test]
async fn test_trades() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    let param = params::PTrade {
        symbol: "BTCUSDT".to_string(),
        limit: 500,
    };
    dbg!(client.trades(param).await.unwrap());
}

#[tokio::test]
async fn test_historical_trades() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    let trade_param = params::PTrade {
        symbol: "BTCUSDT".to_string(),
        limit: 10,
    };
    let trades = client.trades(trade_param).await.unwrap();
    let htrade_param = params::PHistoricalTrade {
        symbol: "BTCUSDT".to_string(),
        from_id: Some(trades.first().unwrap().id),
        limit: None,
    };
    dbg!(client.historical_trades(htrade_param).await.unwrap());
}

#[tokio::test]
async fn test_agg_trades() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    let param = params::PAggTrade {
        symbol: "BTCUSDT".to_string(),
        limit: None,
        from_id: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client.agg_trades(param).await.unwrap());
}

#[tokio::test]
async fn test_klines() {
    let (api_key, secret_key) = init_test();
    let client = UFuturesHttpClient::new(&api_key, &secret_key, BASE_URL);
    let param = params::PKline {
        symbol: "BTCUSDT".to_string(),
        interval: "1m".to_string(),
        start_time: None,
        end_time: None,
        limit: None,
    };
    dbg!(client.klines(param).await.unwrap());
}
