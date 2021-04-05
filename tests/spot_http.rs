use bian_rs::client::spot::*;
use bian_rs::enums::{self, *};
use bian_rs::params;

use std::env;
const BASE_URL: &str = "https://api.binance.com/";

fn init_client() -> SpotHttpClient {
    dotenv::dotenv().unwrap();
    let api_key = env::var("API_KEY").expect("can not find API_KEY env variable");
    let secret_key = env::var("SECRET_KEY").expect("can not find SECRET_KEY env variable");
    SpotHttpClient::new(&api_key, &secret_key, BASE_URL)
}

#[tokio::test]
async fn test_ping() {
    let client = init_client();
    client.ping().await.unwrap();
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
