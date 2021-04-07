use bian_rs::client::*;
use bian_rs::response::WebsocketResponse;
use bian_rs::*;

use std::{env, net::ToSocketAddrs};
const BASE_URL: &str = "wss://fstream.binance.com";

fn init_client() -> UFuturesWSClient {
    dotenv::dotenv().unwrap();
    let proxy = env::var("WS_PROXY").expect("cant not find WS_PROXY env variable");
    let proxy = Some(proxy.to_socket_addrs().unwrap().next().unwrap());
    let base_url = url::Url::parse(BASE_URL).unwrap();
    UFuturesWSClient { proxy, base_url }
}

#[test]
fn test_ws_agg_trade() {
    let client = init_client();
    let mut stream = client.agg_trade("ethusdt".to_string()).unwrap();
    for _ in 0..10 {
        let msg: response::WSAggTrade = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close_stream();

    let mut stream = client
        .agg_trade_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .unwrap();
    for _ in 0..20 {
        let msg: response::WSAggTrade = stream.read_stream_multi().unwrap();
        dbg!(msg);
    }
    stream.close_stream();
}

#[test]
fn test_ws_mark_price() {
    let client = init_client();
    let mut stream = client.mark_price("btcusdt".to_string(), 1).unwrap();
    for _ in 0..5 {
        let msg: response::WSPrice = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close_stream();
    let mut stream = client
        .mark_price_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()], 1)
        .unwrap();
    for _ in 0..10 {
        let msg: response::WSPrice = stream.read_stream_multi().unwrap();
        dbg!(msg);
    }
    stream.close_stream();
}

#[test]
fn test_ws_mark_price_arr() {
    let client = init_client();
    let mut stream = client.mark_price_arr(1).unwrap();
    for _ in 0..3 {
        let msg = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close_stream();
}

#[test]
fn test_ws_kline() {
    let client = init_client();
    let mut stream = client
        .kline("btcusdt".to_string(), enums::Interval::Min1)
        .unwrap();
    for _ in 0..5 {
        dbg!(stream.read_stream_single().unwrap());
    }
    stream.close_stream();
    let mut stream = client
        .kline_multi(
            vec!["btcusdt".to_string(), "ethusdt".to_string()],
            enums::Interval::Min1,
        )
        .unwrap();
    for _ in 0..10 {
        dbg!(stream.read_stream_multi().unwrap());
    }
}

#[test]
fn test_ws_continuous_kline() {
    let client = init_client();
    let mut stream = client
        .continuous_kline(
            "btc".to_string(),
            enums::ContractType::Perpetual,
            enums::Interval::Min1,
        )
        .unwrap();
    for _ in 0..5 {
        dbg!(stream.read_stream_single().unwrap());
    }
    stream.close_stream();
    let mut stream = client
        .continuous_kline_multi(
            vec!["btcusdt".to_string(), "ethusdt".to_string()],
            enums::ContractType::Perpetual,
            enums::Interval::Min1,
        )
        .unwrap();
    for _ in 0..10 {
        dbg!(stream.read_stream_multi().unwrap());
    }
}

#[test]
fn test_ws_mini_ticker() {
    let client = init_client();
    let mut stream = client.mini_ticker("btcusdt".to_string()).unwrap();
    for _ in 0..5 {
        dbg!(stream.read_stream_single().unwrap());
    }
    let mut stream = client
        .mini_ticker_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .unwrap();
    for _ in 0..5 {
        dbg!(stream.read_stream_multi().unwrap());
    }
    let mut stream = client.all_mini_ticker().unwrap();
    for _ in 0..10 {
        dbg!(stream.read_stream_single().unwrap());
    }
}

#[test]
fn test_ws_ticker() {
    let client = init_client();
    let mut stream = client.symbol_ticker("ethusdt".to_string()).unwrap();
    for _ in 0..10 {
        let msg: response::WSTicker = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close_stream();

    let mut stream = client
        .symbol_ticker_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .unwrap();
    for _ in 0..20 {
        let msg: response::WSTicker = stream.read_stream_multi().unwrap();
        dbg!(msg);
    }
    stream.close_stream();

    let mut stream = client.all_symbol_ticker().unwrap();
    for _ in 0..10 {
        dbg!(stream.read_stream_single().unwrap());
    }
    stream.close_stream();
}

#[test]
fn test_ws_book_ticker() {
    let client = init_client();
    let mut stream = client.book_ticker("ethusdt".to_string()).unwrap();
    for _ in 0..10 {
        let msg = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close_stream();

    let mut stream = client
        .book_ticker_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .unwrap();
    for _ in 0..20 {
        let msg = stream.read_stream_multi().unwrap();
        dbg!(msg);
    }
    stream.close_stream();

    let mut stream = client.all_book_ticker().unwrap();
    for _ in 0..10 {
        dbg!(stream.read_stream_single().unwrap());
    }
    stream.close_stream();
}

#[test]
fn test_ws_force_order() {
    let client = init_client();
    let mut stream = client.force_order("ethusdt".to_string()).unwrap();
    for _ in 0..10 {
        let msg = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close_stream();

    let mut stream = client
        .force_order_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .unwrap();
    for _ in 0..20 {
        let msg = stream.read_stream_multi().unwrap();
        dbg!(msg);
    }
    stream.close_stream();

    let mut stream = client.all_force_order().unwrap();
    for _ in 0..10 {
        dbg!(stream.read_stream_single().unwrap());
    }
    stream.close_stream();
}

#[test]
fn test_limit_depth() {
    let client = init_client();
    let mut stream = client.limit_depth("BTCUSDT".to_string(), 5, 100).unwrap();
    for _ in 0..5 {
        dbg!(stream.read_stream_single().unwrap());
    }
    let mut stream = client
        .limit_depth_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()], 5, 100)
        .unwrap();
    for _ in 0..10 {
        dbg!(stream.read_stream_multi().unwrap());
    }
}
