#![feature(async_await)]

use http::{Request, Response};
use hyper::client::HttpConnector;
use hyper::{self, Client};
use interfacer_http::{async_trait, define_custom_fail, HttpClient, HttpService, Url};

// TODO: use generic Connector
pub struct AsyncClient {
    inner: hyper::Client<HttpConnector, hyper::Body>,
}

pub struct AsyncService {
    client: AsyncClient,
    base_url: Url,
}

define_custom_fail!(Error, "hyper error: {}", hyper::Error);

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
