# bian-rs

币安API Rust async SDK

## 完成情况

| 接口      | 现货    | U本位合约 | 币本位合约 | 欧式期权 |
| --------- | ------- | --------- | ---------- | -------- |
| http      | 🚧开发中 | 🆗         | 未开始     | 未开始   |
| websocket | 🚧开发中 | 🆗         | 未开始     | 未开始   |

## 使用

在 `Cargo.toml` 中添加依赖

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
bian-rs = { git = "https://github.com/PrivateRookie/bian-rs.git" }
```

在国内使用需要设置代理，bian-rs 通过 `HTTP_PROXY` 和 `HTTPS_PROXY` 环境变量自动
设置代理。

### http 接口

```rust
use bian_rs::UFuturesHttpClient;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = "your api key";
    let secret_key = "your secret key";
    // 注意末尾的 "/" 必不可少
    let base_url = "https://fapi.binance.com/";
    let client UFuturesHttpClient::new(api_key, secret_key, base_url);
    // 测试是否连通
    client.ping().await.unwrap();
}
```

### websocket 接口


```rust
fn init_client() -> UFuturesWSClient {
    dotenv::dotenv().unwrap();
    let proxy = env::var("WS_PROXY").expect("cant not find WS_PROXY env variable");
    let proxy = Some(proxy.to_socket_addrs().unwrap().next().unwrap());
    let base_url = url::Url::parse(BASE_URL).unwrap();
    UFuturesWSClient { proxy, base_url }
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
}
```

## 贡献代码

因为币安 API 接口众多，若是 bian-rs 还没有实现, 可能需要自己实现。
不过好在bian-rs 使用过程宏 `bian-proc::api` 辅助生成 API 请求函数，你只需要定义API
请求类型，返回数据类型，接着共通过 `#[api(<http_verb> url)]` 即可。

如获取[账户余额](https://binance-docs.github.io/apidocs/futures/cn/#v2-user_data)。

首先在 `src/params.rs` 定义请求参数类型

```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountBalanceV2 {
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}
```

接着在 `src/response.rs` 定义返回数据类型

```rust
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
```

币安返回的数据字段名为 camelCase, 需要通过 `serde(rename_all)` 将其重命名为符合 Rust lint 的 snake_case。
另外币安的某些数据是以字符的保存的浮点数或整数，可以通过 ` #[serde(deserialize_with = "string_as_f64")]` 或 `#[serde(deserialize_with = "string_as_usize")]`
将其直接转换为 f64 或 usize。

接着在 `impl UFuturesHttpClient` 块中使用 `api!` 来实现函数。

`api!` 接受两个参数，http 方法和 url. http 方法除了 `GET`, `POST`, `PUT`, `DELETE` 外，对于需要签名的请求，可以在前面加上 `S`，如 `SGET`。

```rust
/// 账户余额V2
#[api(SGET "fapi/v2/balance")]
pub async fn account_balance_v2(
    &self,
    param: params::AccountBalanceV2,
) -> BianResult<Vec<response::AccountBalance>> {
}
```

接着编写测试用例，确保 url, 请求参数和返回值类型都已定义正确。

```rust
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
```

## TO DO

- [x] websocket 客户端
- [ ] API 限速处理
- [ ] 更多API...

