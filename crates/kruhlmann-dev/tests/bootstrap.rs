use std::time::Duration;

const STATIC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../static");
const TEX_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tex");

use kruhlmann_dev::TexContext;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn connect_with_retry(addr: std::net::SocketAddr) -> TcpStream {
    for _ in 0..100 {
        match TcpStream::connect(addr).await {
            Ok(stream) => return stream,
            Err(_) => tokio::time::sleep(Duration::from_millis(10)).await,
        }
    }
    panic!("could not connect to {addr}");
}

async fn http_get(addr: std::net::SocketAddr, path: &str) -> String {
    let mut stream = connect_with_retry(addr).await;
    let request = format!("GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf).into_owned()
}

#[tokio::test]
async fn bootstrap_serves_index_over_real_tcp_listener() {
    let probe = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = probe.local_addr().unwrap();
    drop(probe);

    let tex_context = TexContext::load_from_directory(TEX_DIR).unwrap();
    let server = tokio::spawn(kruhlmann_dev::bootstrap(addr, STATIC_DIR, tex_context));

    let response = http_get(addr, "/").await;
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    assert!(response.contains("Andreas"));

    server.abort();
}

#[tokio::test]
async fn bootstrap_serves_static_files_over_real_tcp_listener() {
    let probe = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = probe.local_addr().unwrap();
    drop(probe);

    let tex_context = TexContext::load_from_directory(TEX_DIR).unwrap();
    let server = tokio::spawn(kruhlmann_dev::bootstrap(addr, STATIC_DIR, tex_context));

    let response = http_get(addr, "/css/index.css").await;
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    assert!(response.contains("text/css"));

    let response = http_get(addr, "/nope.css").await;
    assert!(response.starts_with("HTTP/1.1 404"), "{response}");

    server.abort();
}

#[tokio::test]
async fn bootstrap_fails_cleanly_on_invalid_address() {
    let tex_context = TexContext::load_from_directory(TEX_DIR).unwrap();
    let result = kruhlmann_dev::bootstrap("256.256.256.256:80", STATIC_DIR, tex_context).await;
    assert!(result.is_err());
}
