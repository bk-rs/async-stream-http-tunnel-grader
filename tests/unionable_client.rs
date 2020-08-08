#[cfg(feature = "async_http1_lite_client")]
#[cfg(test)]
mod unionable_client_with_async_http1_lite_client_tests {
    use std::io;

    use async_net::{TcpListener, TcpStream};
    use async_stream_packed::{Downgrader, Upgrader};
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;
    use futures_lite::stream::StreamExt;

    use async_stream_http_tunnel_grader::{
        async_http1_lite_client::Http1ClientStream, unionable_client::UnionableHttpTunnelStream,
        AsyncHttp1LiteClientHttpTunnelGrader, UnionableHttpTunnelClientGrader,
    };

    #[test]
    fn upgrade() -> io::Result<()> {
        block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let addr = listener.local_addr()?;

            let stream_c = TcpStream::connect(addr).await?;
            let _ = listener
                .incoming()
                .next()
                .await
                .expect("Get next incoming failed")?;

            // TODO, real upgrade
            let mut grader = UnionableHttpTunnelClientGrader::AsyncHttp1Lite(
                AsyncHttp1LiteClientHttpTunnelGrader::new("127.0.0.1".to_owned(), 0, None, None),
            );

            let err = grader.upgrade(stream_c).await.err().unwrap();

            assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);
            assert_eq!(err.to_string(), "read 0");

            Ok(())
        })
    }

    #[test]
    fn downgrade() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo".to_vec());

            // TODO, real downgrade
            let mut grader = UnionableHttpTunnelClientGrader::AsyncHttp1Lite(
                AsyncHttp1LiteClientHttpTunnelGrader::new("localhost".to_owned(), 0, None, None),
            );

            let cursor = grader
                .downgrade(UnionableHttpTunnelStream::AsyncHttp1Lite(
                    Http1ClientStream::new(cursor),
                ))
                .await?;

            assert_eq!(cursor.get_ref(), b"foo");

            Ok(())
        })
    }
}
