#![feature(async_await)]

use http::{Request, Response};
use hyper::client::HttpConnector;
use hyper::{self, Client};
use interfacer_http_service::{async_trait, HttpClient, HttpService, RequestFail, Url};
use std::fmt::Display;
// use hyper::body::Payload;
// use std::pin::Pin;
// use std::task::{Context, Poll};

// TODO: use generic Connector
pub struct AsyncClient {
    inner: hyper::Client<HttpConnector, hyper::Body>,
}

pub struct AsyncService {
    client: AsyncClient,
    base_url: Url,
}

#[derive(Debug)]
pub struct Error(hyper::Error);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {

}

impl From<Error> for RequestFail {
    fn from(err: Error) -> Self {
        RequestFail::http(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error(err)
    }
}

// pub struct Body {
//     inner: hyper::Body,
// }

impl AsyncClient {
    pub fn new() -> Self {
        Self {
            inner: Client::new(),
        }
    }
}

impl AsyncService {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: AsyncClient::new(),
            base_url,
        }
    }
}

// impl Body {
//     pub fn new(body: hyper::Body) -> Self {
//         Self { inner: body }
//     }
// }

// impl From<hyper::Body> for Body {
//     fn from(body: hyper::Body) -> Self {
//         Body::new(body)
//     }
// }

// impl AsyncRead for Body {
//     fn poll_read(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         buf: &mut [u8],
//     ) -> Poll<std::io::Result<usize>> {
//         match self.get_mut().inner.poll_read(cx) {}
//     }
// }

#[async_trait]
impl HttpClient for AsyncClient {
    type Err = Error;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Err> {
        let (parts, body) = req.into_parts();
        let (parts, mut body) = self
            .inner
            .request(Request::from_parts(parts, body.into()))
            .await?
            .into_parts();
        let mut data = Vec::new();
        while let Some(chunk) = body.next().await {
            data.extend_from_slice(&chunk?);
        }
        Ok(Response::from_parts(parts, data))
    }
}

impl HttpService for AsyncService {
    type Client = AsyncClient;

    fn get_base_url(&self) -> &Url {
        &self.base_url
    }

    fn get_client(&self) -> &Self::Client {
        &self.client
    }
}
