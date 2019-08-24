pub extern crate hyper;
pub use error::{Error, Result};

use hyper::client::connect::{Connect, HttpConnector};
use interfacer_http::http::{Request, Response};
use interfacer_http::{async_trait, Helper, HttpClient};
mod error;

#[derive(Clone)]
pub struct Client<C> {
    inner: hyper::Client<C, hyper::Body>,
    helper: Helper,
}

impl Client<HttpConnector> {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
            helper: Default::default(),
        }
    }
}

impl<C> Client<C> {
    pub fn base_on(client: hyper::Client<C, hyper::Body>) -> Self {
        Self {
            inner: client,
            helper: Default::default(),
        }
    }

    pub fn with_helper(self, helper: Helper) -> Self {
        Self { helper, ..self }
    }
}

#[async_trait]
impl<C> HttpClient for Client<C>
where
    C: Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
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
