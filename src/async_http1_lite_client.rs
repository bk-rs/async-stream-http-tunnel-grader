use std::io;

pub use async_http1_lite::message::http::{HeaderMap, HeaderValue};
use async_http1_lite::{
    message::http::{
        header::{ACCEPT, HOST, PROXY_AUTHORIZATION, USER_AGENT},
        Method, Version,
    },
    Http1ClientStream, Request,
};
use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use async_stream_packed::{Downgrader, HttpTunnelGrader, Upgrader, UpgraderExtRefer};

use crate::authorization::Authorization;

pub struct AsyncHttp1LiteClientHttpTunnelGrader {
    host: String,
    port: u16,
    authorization: Option<Authorization>,
    headers: Option<HeaderMap<HeaderValue>>,
}
impl AsyncHttp1LiteClientHttpTunnelGrader {
    pub fn new(
        host: String,
        port: u16,
        authorization: Option<Authorization>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> Self {
        Self {
            host,
            port,
            authorization,
            headers,
        }
    }
}

#[async_trait]
impl<S> Upgrader<S> for AsyncHttp1LiteClientHttpTunnelGrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = Http1ClientStream<S>;
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
        let mut stream = Http1ClientStream::new(stream);

        let authority = format!("{}:{}", self.host, self.port);

        let mut request = Request::builder()
            .method(Method::CONNECT)
            .uri(authority.as_str())
            .version(Version::HTTP_11)
            .body(vec![])
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        if let Some(authorization) = &self.authorization {
            request.headers_mut().insert(
                PROXY_AUTHORIZATION,
                authorization
                    .header_value()
                    .parse()
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?,
            );
        }

        let mut headers = self.headers.clone().unwrap_or_default();
        headers.insert(
            HOST,
            HeaderValue::from_str(authority.as_str())
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?,
        );
        if headers.get(USER_AGENT).is_none() {
            headers.insert(USER_AGENT, HeaderValue::from_static("curl/7.71.1"));
        }
        if headers.get(ACCEPT).is_none() {
            headers.insert(ACCEPT, HeaderValue::from_static("*"));
        }
        headers.insert("Proxy-Connection", HeaderValue::from_static("Keep-Alive"));

        for (k, v) in headers.iter() {
            request.headers_mut().insert(k, v.to_owned());
        }

        stream.write_request(request).await?;

        let (response, _) = stream.read_response().await?;
        if !response.status().is_success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "response status mismatch",
            ));
        }

        Ok(stream)
    }
}

#[async_trait]
impl<S> Downgrader<S> for AsyncHttp1LiteClientHttpTunnelGrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    async fn downgrade(&mut self, output: <Self as Upgrader<S>>::Output) -> io::Result<S> {
        output.into_inner()
    }
}

impl<S> HttpTunnelGrader<S> for AsyncHttp1LiteClientHttpTunnelGrader where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
}

impl<S> UpgraderExtRefer<S> for AsyncHttp1LiteClientHttpTunnelGrader
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
