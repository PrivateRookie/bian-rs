use bian_rs::response::WebsocketResponse;
use bian_rs::*;

use std::{env, net::ToSocketAddrs};
const BASE_URL: &str = "wss://fstream.binance.com";

fn init_client() -> UFuturesProxyWSClient {
    dotenv::dotenv().unwrap();
    let proxy = env::var("WS_PROXY").expect("cant not find WS_PROXY env variable");
    let proxy = proxy.to_socket_addrs().unwrap().next().unwrap();
    let base_url = url::Url::parse(BASE_URL).unwrap();
    UFuturesProxyWSClient { proxy, base_url }
}

#[test]
fn test_ws_agg_trade() {
    let client = init_client();
    let mut stream = client.agg_trade("ethusdt".to_string()).unwrap();
    for _ in 0..10 {
        let msg: response::WSAggTrade = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close(None).unwrap();

    let mut stream = client
        .agg_trade_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .unwrap();
    for _ in 0..20 {
        let msg: response::WSAggTrade = stream.read_stream_multi().unwrap();
        dbg!(msg);
    }
    stream.close(None).unwrap();
}

#[test]
fn test_ws_ticker() {
    let client = init_client();
    let mut stream = client.symbol_ticker("ethusdt".to_string()).unwrap();
    for _ in 0..10 {
        let msg: response::Ticker = stream.read_stream_single().unwrap();
        dbg!(msg);
    }
    stream.close(None).unwrap();

    let mut stream = client
        .symbol_ticker_multi(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .unwrap();
    for _ in 0..20 {
        let msg: response::Ticker = stream.read_stream_multi().unwrap();
        dbg!(msg);
    }
    stream.close(None).unwrap();
}
