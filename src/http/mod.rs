pub use http::{
    header, header::HeaderMap, method, method::Method, request, request::Request, response,
    response::Response, status, status::StatusCode, uri, uri::Uri, version, version::Version,
    HttpTryFrom,
};

pub trait HttpService {
    fn get_base_url(&self) -> &str;
}

pub trait FromContent<const CONTENT_TYPE: &'static str> {
    fn from_content(data: &[u8]) -> Self;
}

pub trait ToContent<const CONTENT_TYPE: &'static str> {
    fn to_content(&self) -> &[u8];
}

pub mod content_type;
