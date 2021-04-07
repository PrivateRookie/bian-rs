use bian_rs::client::*;
use bian_rs::enums::{self, *};
use bian_rs::params;

use std::env;
const BASE_URL: &str = "https://dapi.binance.com/";

fn init_client() -> DFuturesHttpClient {
    dotenv::dotenv().unwrap();
    let api_key = env::var("API_KEY").expect("can not find API_KEY env variable");
    let secret_key = env::var("SECRET_KEY").expect("can not find SECRET_KEY env variable");
    DFuturesHttpClient::new(&api_key, &secret_key, BASE_URL)
}

#[tokio::test]
async fn test_ping() {
    let client = init_client();
    client.ping().await.unwrap();
}

#[tokio::test]
async fn test_balance() {
    let client = init_client();
    let params = params::PTimestamp::now();
    client.account_balance_v2(params).await.unwrap();
}

#[tokio::test]
async fn test_server_time() {
    let client = init_client();
    client.server_time().await.unwrap();
}
#[tokio::test]
async fn test_exchange_info() {
    let client = init_client();
    client.exchange_info().await.unwrap();
}

#[tokio::test]
async fn test_depth() {
    let client = init_client();
    let param = params::PDepth {
        symbol: "BTCUSD_PERP".to_string(),
        limit: 500,
    };
    dbg!(client.depth(param).await.unwrap());
}

#[tokio::test]
async fn test_trades() {
    let client = init_client();
    let param = params::PTrade {
        symbol: "BTCUSD_PERP".to_string(),
        limit: 500,
    };
    dbg!(client.trades(param).await.unwrap());
}

#[tokio::test]
async fn test_historical_trades() {
    let client = init_client();
    let trade_param = params::PTrade {
        symbol: "BTCUSD_PERP".to_string(),
        limit: 10,
    };
    let trades = client.trades(trade_param).await.unwrap();
    let htrade_param = params::PHistoricalTrade {
        symbol: "BTCUSD_PERP".to_string(),
        from_id: Some(trades.first().unwrap().id),
        limit: None,
    };
    dbg!(client.historical_trades(htrade_param).await.unwrap());
}

#[tokio::test]
async fn test_agg_trades() {
    let client = init_client();
    let param = params::PAggTrade {
        symbol: "BTCUSD_PERP".to_string(),
        limit: None,
        from_id: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client.agg_trades(param).await.unwrap());
}

#[tokio::test]
async fn test_klines() {
    let client = init_client();
    let param = params::PKline {
        symbol: "BTCUSD_PERP".to_string(),
        interval: enums::Interval::Min1,
        start_time: None,
        end_time: None,
        limit: None,
    };
    dbg!(client.klines(param).await.unwrap());
}

#[tokio::test]
async fn test_continuous_klines() {
    let client = init_client();
    let param = params::PContinuousKline {
        pair: "BTCUSD_PERP".to_string(),
        interval: enums::Interval::Min1,
        start_time: None,
        end_time: None,
        limit: None,
        contract_type: "PERPETUAL".to_string(),
    };
    dbg!(client.continuous_klines(param).await.unwrap());
}

#[tokio::test]
async fn test_index_price_klines() {
    let client = init_client();
    let param = params::PContinuousKline {
        pair: "BTCUSD_PERP".to_string(),
        interval: enums::Interval::Min1,
        start_time: None,
        end_time: None,
        limit: None,
        contract_type: "PERPETUAL".to_string(),
    };
    dbg!(client.continuous_klines(param).await.unwrap());
}

#[tokio::test]
async fn test_premium_index() {
    let client = init_client();
    let param = params::PSymbolPair {
        symbol: None,
        pair: Some("BTCUSD".to_string()),
    };
    dbg!(client.premium_index(param).await.unwrap());
}

#[tokio::test]
async fn test_premium_indexes() {
    let client = init_client();
    dbg!(client.premium_indexes().await.unwrap());
}

#[tokio::test]
async fn test_funding_rate() {
    let client = init_client();
    let param = params::PFundingRate {
        symbol: Some("BTCUSD_PERP".to_string()),
        start_time: None,
        end_time: None,
        limit: Some(3),
    };
    dbg!(client.funding_rate(param).await.unwrap());
}

#[tokio::test]
async fn test_h24_ticker() {
    let client = init_client();
    let param = params::PSymbolPair {
        symbol: None,
        pair: Some("BTCUSD".to_string()),
    };
    dbg!(client.h24_ticker(param).await.unwrap());
}

#[tokio::test]
async fn test_h24_tickers() {
    let client = init_client();
    dbg!(client.h24_tickers().await.unwrap());
}

#[tokio::test]
async fn test_price() {
    let client = init_client();
    let param = params::PSymbolPair {
        symbol: None,
        pair: Some("BTCUSD".to_string()),
    };
    dbg!(client.price(param).await.unwrap());
}

#[tokio::test]
async fn test_prices() {
    let client = init_client();
    dbg!(client.prices().await.unwrap());
}

#[tokio::test]
async fn test_book_ticker() {
    let client = init_client();
    let param = params::PSymbolPair {
        symbol: None,
        pair: Some("BTCUSD".to_string()),
    };
    dbg!(client.book_ticker(param).await.unwrap());
}

#[tokio::test]
async fn test_book_tickers() {
    let client = init_client();
    dbg!(client.book_tickers().await.unwrap());
}

#[tokio::test]
async fn test_all_force_order() {
    let client = init_client();
    let param = params::PForceOrder {
        symbol: Some("BTCUSD_PERP".to_string()),
        start_time: None,
        end_time: None,
        limit: Some(10),
    };
    dbg!(client.all_force_orders(param).await.unwrap());
}

#[tokio::test]
async fn test_open_interest() {
    let client = init_client();
    let param = params::PSymbol {
        symbol: "BTCUSD_PERP".to_string(),
    };
    dbg!(client.open_interest(param).await.unwrap());
}

#[tokio::test]
async fn test_open_interest_hist() {
    let client = init_client();
    let param = params::PFutures {
        symbol: "BTCUSD_PERP".to_string(),
        period: Interval::Min1,
        limit: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client.open_interest_hist(param).await.unwrap());
}
#[tokio::test]
async fn test_top_long_short_account_ratio() {
    let client = init_client();
    let param = params::PFutures {
        symbol: "BTCUSD_PERP".to_string(),
        period: Interval::Min1,
        limit: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client.top_long_short_account_ratio(param).await.unwrap());
}

#[tokio::test]
async fn test_top_long_short_position_ratio() {
    let client = init_client();
    let param = params::PFutures {
        symbol: "BTCUSD_PERP".to_string(),
        period: Interval::Min1,
        limit: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client.top_long_short_position_ratio(param).await.unwrap());
}
#[tokio::test]
async fn test_global_long_short_position_ratio() {
    let client = init_client();
    let param = params::PFutures {
        symbol: "BTCUSD_PERP".to_string(),
        period: Interval::Min1,
        limit: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client
        .global_long_short_position_ratio(param)
        .await
        .unwrap());
}

#[tokio::test]
async fn test_taker_long_short_ratio() {
    let client = init_client();
    let param = params::PFutures {
        symbol: "BTCUSD_PERP".to_string(),
        period: Interval::Min1,
        limit: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client.taker_long_short_ratio(param).await.unwrap());
}

#[tokio::test]
async fn test_lvt_klines() {
    let client = init_client();
    let param = params::PLvtKlines {
        symbol: "BTCUP".to_string(),
        interval: Interval::Min1,
        limit: None,
        start_time: None,
        end_time: None,
    };
    dbg!(client.lvt_klines(param).await.unwrap());
}

#[tokio::test]
async fn test_update_position_side() {
    let client = init_client();
    let now = chrono::Utc::now();
    let param = params::PPositionSideDual {
        dual_side_position: false,
        ts: params::PTimestamp {
            timestamp: now.timestamp_millis(),
            recv_window: None,
        },
    };
    dbg!(client.update_position_side(param).await.unwrap());
}

#[tokio::test]
async fn test_position_side() {
    let client = init_client();
    let now = chrono::Utc::now();
    let param = params::PTimestamp {
        timestamp: now.timestamp_millis(),
        recv_window: None,
    };
    dbg!(client.get_position_side(param).await.unwrap());
}

#[tokio::test]
async fn test_order_test() {
    let client = init_client();
    let ts = params::PTimestamp::now();
    let param = params::PFuturesOrder {
        spec: params::PFuturesOrderSpec {
            symbol: "adausdt".to_string(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: FuturesOrderType::Market,
            reduce_only: Some(false),
            quantity: Some(10.0),
            price: None,
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            time_in_force: None,
            working_type: None,
            price_protect: None,
            new_order_resp_type: None,
        },
        ts,
    };
    let resp = client.order(param).await.unwrap();
    dbg!(resp);
}

#[tokio::test]
async fn test_listen_key() {
    let client = init_client();
    let key = client.create_listen_key().await.unwrap();
    dbg!(&key);
    client.update_listen_key().await.unwrap();
    client.close_listen_key().await.unwrap();
}
