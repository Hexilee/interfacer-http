pub use http::{
    header, header::HeaderMap, method, method::Method, request, request::Request, response,
    response::Response, status, status::StatusCode, uri, uri::Uri, version, version::Version,
    HttpTryFrom,
};

use core::future::Future;

pub trait HttpClient {
    type Response: Future;
    type Body;
    fn request(&self, req: Request<Self::Body>) -> Self::Response;
}

pub trait HttpService {
    type Client: HttpClient;
    fn get_base_url(&self) -> &Uri;
    fn get_client(&self) -> &Self::Client;
}

pub trait FromContent<const CONTENT_TYPE: &'static str> {
    fn from_content(data: &[u8]) -> Self;
}

pub trait ToContent<const CONTENT_TYPE: &'static str> {
    fn to_content(&self) -> &[u8];
}

pub mod content_type;
