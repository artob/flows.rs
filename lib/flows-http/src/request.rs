// This is free and unencumbered software released into the public domain.

use super::Result;
use alloc::boxed::Box;
use async_flow::{Inputs, Outputs};
use core::error::Error as StdError;
use hyper::body::{Body, Incoming};

/// A block that outputs HTTP responses corresponding to input HTTP requests.
pub async fn request<T>(
    mut requests: Inputs<http::Request<T>>,
    responses: Outputs<Result<http::Response<Incoming>>>,
) -> Result<(), async_flow::Error>
where
    T: Body + Send + 'static + Unpin,
    T::Data: Send,
    T::Error: Into<Box<dyn core::error::Error + Send + Sync>>,
{
    while let Some(request) = requests.recv().await? {
        let response = execute(request).await;
        responses.send(response).await?;
    }
    Ok(())
}

#[cfg(all(
    feature = "std",
    all(feature = "http1", not(feature = "http2")),
    not(feature = "tls")
))]
async fn execute<T>(request: http::Request<T>) -> Result<http::Response<Incoming>>
where
    T: Body + Send + 'static,
    T::Data: Send,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    use super::Error;
    use hyper::client::conn::http1;
    use hyper_util::rt::TokioIo;
    use tokio::net::TcpStream;

    let url = request.uri();
    let url_scheme = url.scheme().ok_or(Error::MissingUrlScheme)?;
    let url_host = url.host().ok_or(Error::MissingUrlHost)?;
    let is_https = url_scheme == &http::uri::Scheme::HTTPS;
    let url_port = url
        .port_u16()
        .unwrap_or_else(|| if is_https { 443 } else { 80 });

    let tcp_addr = (url_host, url_port);
    let tcp_stream = match TcpStream::connect(tcp_addr).await {
        Err(error) => return Err(Error::TcpConnectFailed(error)),
        Ok(tcp_stream) => tcp_stream,
    };

    let io_adapter = TokioIo::new(tcp_stream);
    let mut sender = match http1::handshake(io_adapter).await {
        Err(error) => return Err(Error::HttpHandshakeFailed(error)),
        Ok((sender, conn)) => {
            tokio::task::spawn(async move {
                if let Err(error) = conn.await {
                    #[cfg(feature = "std")]
                    std::eprintln!("Connection failed: {:?}", error); // FIXME
                }
            });
            sender
        },
    };

    Ok(sender.send_request(request).await?)
}

#[cfg(all(
    feature = "std",
    any(feature = "http1", feature = "http2"),
    feature = "tls"
))]
async fn execute<T>(request: http::Request<T>) -> Result<http::Response<Incoming>>
where
    T: Body + Send + 'static + Unpin,
    T::Data: Send,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    use super::Error;
    use hyper_util::{client::legacy::Client, rt::TokioExecutor};
    use rustls::ClientConfig;
    use rustls_platform_verifier::ConfigVerifierExt;

    let url = request.uri();
    let _ = url.scheme().ok_or(Error::MissingUrlScheme)?;
    let _ = url.host().ok_or(Error::MissingUrlHost)?;

    let tls_config = ClientConfig::with_platform_verifier();
    let http_connector = hyper_rustls::HttpsConnectorBuilder::new().with_tls_config(tls_config);

    #[cfg(all(feature = "http1", not(feature = "http2")))]
    let http_connector = http_connector.https_or_http().enable_http1();

    #[cfg(all(feature = "http2", not(feature = "http1")))]
    let http_connector = http_connector.https_only().enable_http2();

    #[cfg(all(feature = "http1", feature = "http2"))]
    let http_connector = http_connector.https_or_http().enable_http1().enable_http2();

    let http_client: Client<_, T> =
        Client::builder(TokioExecutor::new()).build(http_connector.build());

    Ok(http_client
        .request(request)
        .await
        .map_err(|_e| Error::Other(Box::new(_e)))?)
}

#[cfg(any(not(feature = "std"), not(any(feature = "http1", feature = "http2"))))]
async fn execute<T>(_request: http::Request<T>) -> Result<http::Response<Incoming>>
where
    T: Body + Send + 'static,
    T::Data: Send,
    T::Error: Into<Box<dyn core::error::Error + Send + Sync>>,
{
    #[allow(unreachable_code)]
    return Err(unimplemented!());
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{boxed::Box, string::String};
    use async_flow::{Channel, InputPort};
    use core::error::Error;

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[tokio::test]
    async fn test_request() -> Result<(), Box<dyn Error>> {
        let mut in_ = Channel::bounded(1);
        let mut out = Channel::bounded(1);

        let fetcher = tokio::spawn(request(in_.rx, out.tx));

        #[cfg(all(feature = "http1", not(feature = "http2")))]
        let urls = ["http://httpbin.org/ip"];

        #[cfg(feature = "http2")]
        let urls = ["https://ar.to/robots.txt"];

        for url in urls {
            use hyper::header::HOST;
            let url = url
                .parse::<http::Uri>()
                .expect("the input URL should be valid");
            let url_authority = url
                .authority()
                .expect("the input URL should have an authority segment")
                .clone();
            let request = http::Request::builder()
                .uri(url)
                .header(HOST, url_authority.as_str())
                .body(String::new())
                .expect("the HTTP request should be constructed");

            in_.tx.send(request).await.unwrap();
        }
        in_.tx.close();

        let _ = tokio::join!(fetcher);

        let outputs = out.rx.recv_all().await.unwrap();
        #[cfg(feature = "std")]
        std::eprintln!("{:?}", outputs); // DEBUG
        assert_eq!(outputs.len(), 1);

        Ok(())
    }
}
