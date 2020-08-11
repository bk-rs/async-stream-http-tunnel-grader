/*
cargo run -p async-stream-http-tunnel-grader-demo-async-net --bin async_http1_lite_client 127.0.0.1 8118 httpbin.org 80 /ip
*/

use std::env;
use std::io;
use std::str;

use async_net::TcpStream;
use async_stream_packed::{Downgrader, Upgrader};
use futures_lite::future::block_on;
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use async_stream_http_tunnel_grader::AsyncHttp1LiteClientHttpTunnelGrader;

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let proxy_domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("PROXY_DOMAIN").unwrap_or("127.0.0.1".to_owned()));
    let proxy_port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PROXY_PORT").unwrap_or("8118".to_owned()))
        .parse()
        .unwrap();
    let domain = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("httpbin.org".to_owned()));
    let port: u16 = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("80".to_owned()))
        .parse()
        .unwrap();
    let uri = env::args()
        .nth(5)
        .unwrap_or_else(|| env::var("URI").unwrap_or("/ip".to_owned()));

    println!(
        "async_http1_lite_client {} {} {} {} {}",
        proxy_domain, proxy_port, domain, port, uri
    );

    //
    let addr = format!("{}:{}", proxy_domain, proxy_port);
    let stream = TcpStream::connect(addr).await?;

    let mut grader = AsyncHttp1LiteClientHttpTunnelGrader::new(domain.clone(), port, None, None);

    let http_stream = grader.upgrade(stream).await?;
    let mut stream = grader.downgrade(http_stream).await?;

    let req_string = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n",
        uri, domain
    );
    println!("{}", req_string);

    stream.write(&req_string.as_bytes()).await?;

    let mut buf = vec![0u8; 288];
    stream.read(&mut buf).await?;

    println!("{:?}", str::from_utf8(&buf));

    println!("done");

    Ok(())
}
