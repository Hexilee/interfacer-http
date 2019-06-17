use crate::http::HttpClient;
use http::Request;
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::{Body, Client};

pub struct AsyncClient {
    inner: hyper::Client<HttpConnector, Body>,
}

impl AsyncClient {
    pub fn new() -> Self {
        Self {
            inner: Client::new(),
        }
    }
}

impl HttpClient for AsyncClient {
    type Response = ResponseFuture;
    type Body = Vec<u8>;
    fn request(&self, req: http::Request<Self::Body>) -> Self::Response {
        let (parts, data) = req.into_parts();
        self.inner.request(Request::from_parts(parts, data.into()))
    }
    fn _phantom(&self) -> Self::Body {
        unimplemented!()
    }
}
