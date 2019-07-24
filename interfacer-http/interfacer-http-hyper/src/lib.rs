#![feature(async_await)]

use http::{Request, Response, Uri};
use hyper::client::HttpConnector;
use hyper::{self, Client, Error};
use interfacer_http_service::{async_trait, HttpClient, HttpService};
// use hyper::body::Payload;
// use std::pin::Pin;
// use std::task::{Context, Poll};

// TODO: use generic Connector
pub struct AsyncClient {
    inner: hyper::Client<HttpConnector, hyper::Body>,
}

pub struct AsyncService {
    client: AsyncClient,
    base_uri: Uri,
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
    pub fn new(base_uri: Uri) -> Self {
        Self {
            client: AsyncClient::new(),
            base_uri,
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

    fn get_base_url(&self) -> &Uri {
        &self.base_uri
    }

    fn get_client(&self) -> &Self::Client {
        &self.client
    }
}
