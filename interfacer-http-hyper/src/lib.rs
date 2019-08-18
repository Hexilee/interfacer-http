#![feature(async_await)]

use hyper::client::HttpConnector;
use interfacer_http::http::{Request, Response};
use interfacer_http::{async_trait, Helper, HttpClient};
mod error;

pub use error::{Error, Result};

// TODO: use generic Connector
#[derive(Clone)]
pub struct Client {
    inner: hyper::Client<HttpConnector, hyper::Body>,
    helper: Helper,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: hyper::Client::new(),
            helper: Default::default(),
        }
    }

    pub fn with_helper(helper: Helper) -> Self {
        Self {
            inner: hyper::Client::new(),
            helper,
        }
    }
}

#[async_trait]
impl HttpClient for Client {
    type Err = Error;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
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

    fn helper(&self) -> &Helper {
        &self.helper
    }
}
