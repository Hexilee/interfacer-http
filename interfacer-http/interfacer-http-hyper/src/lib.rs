#![feature(async_await)]

use http::{Request, Response, Uri};
use hyper::client::HttpConnector;
use hyper::{Body, Client, Error};
use interfacer_http_service::{async_trait, HttpClient, HttpService};

pub struct AsyncClient {
    inner: hyper::Client<HttpConnector, Body>,
}

pub struct AsyncService {
    client: AsyncClient,
    base_uri: Uri,
}

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

#[async_trait]
impl HttpClient for AsyncClient {
    type Err = Error;
    type Body = Body;
    async fn request(&self, req: Request<Self::Body>) -> Result<Response<Self::Body>, Self::Err> {
        self.inner.request(req).await
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
