use http::{Request, Uri};
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::{Body, Client};
use interfacer_http_service::{HttpClient, HttpService};

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

// TODO: send request
impl HttpClient for AsyncClient {
    type Response = ResponseFuture;
    type Body = Vec<u8>;
    fn request(&self, req: http::Request<Self::Body>) -> Self::Response {
        let (parts, data) = req.into_parts();
        self.inner.request(Request::from_parts(parts, data.into()))
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
