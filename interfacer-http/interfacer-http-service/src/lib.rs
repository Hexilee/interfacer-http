pub use http::{
    header, header::HeaderMap, method, method::Method, request, request::Request, response,
    response::Response, status, status::StatusCode, uri, uri::Uri, version, version::Version,
    HttpTryFrom,
};

pub use async_trait::async_trait;

#[async_trait]
pub trait HttpClient {
    type Err;
    type Body;
    async fn request(&self, req: Request<Self::Body>) -> Result<Response<Self::Body>, Self::Err>;
}

pub trait HttpService {
    type Client: HttpClient;
    fn get_base_url(&self) -> &Uri;
    fn get_client(&self) -> &Self::Client;
}

pub trait FromContent: Sized {
    const CONTENT_TYPE: &'static str;
    const CHARSET: &'static str = content_type::CHARSET_UTF8;
    type Err;
    fn from_content(data: &[u8]) -> Result<Self, Self::Err>;
}

pub trait IntoContent {
    const CONTENT_TYPE: &'static str;
    const CHARSET: &'static str = content_type::CHARSET_UTF8;
    fn into_content(&self) -> &[u8];
}

pub mod content_type;
