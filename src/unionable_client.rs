use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use async_stream_packed::{Downgrader, HttpTunnelClientGrader, Upgrader, UpgraderExtRefer};

pub enum UnionableHttpTunnelClientGrader {
    #[cfg(feature = "async_http1_lite_client")]
    AsyncHttp1Lite(crate::async_http1_lite_client::AsyncHttp1LiteClientHttpTunnelGrader),
}

#[async_trait]
impl<S> Upgrader<S> for UnionableHttpTunnelClientGrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = UnionableHttpTunnelStream<S>;
    #[allow(unused_variables)]
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
        match self {
            #[cfg(feature = "async_http1_lite_client")]
            UnionableHttpTunnelClientGrader::AsyncHttp1Lite(grader) => {
                let stream = grader.upgrade(stream).await?;
                Ok(UnionableHttpTunnelStream::AsyncHttp1Lite(stream))
            }
            #[cfg(all(not(feature = "async_http1_lite_client")))]
            _ => unreachable!(),
        }
    }
}

#[async_trait]
impl<S> Downgrader<S> for UnionableHttpTunnelClientGrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    #[allow(unused_variables)]
    async fn downgrade(&mut self, output: <Self as Upgrader<S>>::Output) -> io::Result<S> {
        match self {
            #[cfg(feature = "async_http1_lite_client")]
            UnionableHttpTunnelClientGrader::AsyncHttp1Lite(grader) => match output {
                #[cfg(feature = "async_http1_lite_client")]
                UnionableHttpTunnelStream::AsyncHttp1Lite(stream) => grader.downgrade(stream).await,
            },
            #[cfg(all(not(feature = "async_http1_lite_client")))]
            _ => unreachable!(),
        }
    }
}

#[async_trait]
impl<S> HttpTunnelClientGrader<S> for UnionableHttpTunnelClientGrader where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
}

impl<S> UpgraderExtRefer<S> for UnionableHttpTunnelClientGrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    fn get_ref(output: &Self::Output) -> &S {
        output.get_ref()
    }

    fn get_mut(output: &mut Self::Output) -> &mut S {
        output.get_mut()
    }
}

//
//
//
pub enum UnionableHttpTunnelStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    #[cfg(feature = "async_http1_lite_client")]
    AsyncHttp1Lite(crate::async_http1_lite_client::Http1ClientStream<S>),
    #[cfg(all(not(feature = "async_http1_lite_client")))]
    Never(std::marker::PhantomData<S>),
}

macro_rules! unionable_http_tunnel_stream {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            #[cfg(feature = "async_http1_lite_client")]
            UnionableHttpTunnelStream::AsyncHttp1Lite($pattern) => $result,
            #[cfg(all(not(feature = "async_http1_lite_client")))]
            UnionableHttpTunnelStream::Never(_) => unreachable!(),
        }
    };
}

#[allow(unused_variables)]
impl<S> UnionableHttpTunnelStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    fn get_ref(&self) -> &S {
        unionable_http_tunnel_stream!(self, s => s.get_ref())
    }
    fn get_mut(&mut self) -> &mut S {
        unionable_http_tunnel_stream!(self, s => s.get_mut())
    }
}

#[allow(unused_variables)]
impl<S> AsyncRead for UnionableHttpTunnelStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        unionable_http_tunnel_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_read(cx, buf))
    }
}

#[allow(unused_variables)]
impl<S> AsyncWrite for UnionableHttpTunnelStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        unionable_http_tunnel_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_write(cx, buf))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        unionable_http_tunnel_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_flush(cx))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        unionable_http_tunnel_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_close(cx))
    }
}
