use std::env;

use http_body_util::BodyExt;
use hyper::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::TcpStream;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

use base64::prelude::{Engine as _, BASE64_STANDARD};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: JsonValue,
    id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    // Create the Basic Authentication header
    let auth = format!("{}:{}", "dogecoin", "doge");
    let encoded_auth = BASE64_STANDARD.encode(&auth);
    let auth_header_value = format!("Basic {}", encoded_auth);

    let req_body = RpcRequest {
        jsonrpc: "2.0".to_owned(),
        method: "getblock".to_owned(),
        params: json!([
            "82bc68038f6034c0596b6e313729793a887fded6e92a31fbdf70863f89d9bea2",
            true
        ]),
        id: 1,
    };

    let req_body = serde_json::to_string(&req_body)
        .map_err(|err| format!("Failed to serialize request body: {}", err))?;

    let path = url.path().to_owned();
    let req = Request::builder()
        .method("POST")
        .uri(path)
        // .header(hyper::header::HOST, authority.as_ref())
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(AUTHORIZATION, HeaderValue::from_str(&auth_header_value)?)
        .body(req_body)?;

    println!("Request: {:#?}", req);

    let mut res = sender
        .send_request(req)
        .await
        .map_err(|err| format!("Failed to send request: {}", err))?;

    // println!("Response: {}", res.status());
    // println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(&chunk).await?;
        }
    }

    // println!("\n\nDone!");

    Ok(())
}
