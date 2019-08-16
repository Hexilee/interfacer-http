#![feature(async_await)]

use failure::Fail;
use http::{Request, Response};
use hyper::client::HttpConnector;
use interfacer_http::{async_trait, define_from, url::Url, HttpClient, HttpService};

// TODO: use generic Connector
#[derive(Clone)]
pub struct Client {
    inner: hyper::Client<HttpConnector, hyper::Body>,
}

#[derive(Clone)]
pub struct Service {
    client: Client,
    base_url: Url,
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "hyper error: {}", err)]
    Hyper { err: hyper::Error },
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper { err }
    }
}

define_from!(Error);

impl Client {
    pub fn new() -> Self {
        Self {
            inner: hyper::Client::new(),
        }
    }
}

impl Service {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    fn __check() {
        let _: Box<dyn HttpService<Client = Client>> = Box::new(Service::new("".parse().unwrap()));
    }
}

#[async_trait]
impl HttpClient for Client {
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

impl HttpService for Service {
    type Client = Client;

    fn get_base_url(&self) -> &Url {
        &self.base_url
    }

    fn get_client(&self) -> &Self::Client {
        &self.client
    }
}
