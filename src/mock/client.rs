use super::error::{Error, Result};
use crate::{
    async_trait,
    http::{Request, Response},
    url::Url,
    Helper, HttpClient,
};
use std::future::Future;

pub struct Client<F> {
    helper: Helper,
    handler: fn(Request<Vec<u8>>) -> F,
}

impl<F> Client<F>
where
    F: Future<Output = Result<Response<Vec<u8>>>> + Send + 'static,
{
    pub fn new(base_url: Url, handler: fn(Request<Vec<u8>>) -> F) -> Self {
        Self {
            handler,
            helper: Helper::new().with_base_url(base_url),
        }
    }
}

#[async_trait]
impl<F> HttpClient for Client<F>
where
    F: Future<Output = Result<Response<Vec<u8>>>> + Send + 'static,
{
    type Err = Error;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
        (self.handler)(req).await
    }

    fn helper(&self) -> &Helper {
        &self.helper
    }
}
