pub use http::{
    header, header::HeaderMap, method, method::Method, request, request::Request, response,
    response::Response, status, status::StatusCode, uri, uri::Uri, version, version::Version,
    HttpTryFrom,
};

pub use futures::{
    AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt
};

pub use async_trait::async_trait;

// TODO: use T: AsyncRead as type of Request::Body
#[async_trait]
pub trait HttpClient {
    type Err;
    type Body: AsyncRead;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Self::Body>, Self::Err>;
}

pub trait HttpService {
    type Client: HttpClient;
    fn get_base_url(&self) -> &Uri;
    fn get_client(&self) -> &Self::Client;
}

// TODO: use T: AsyncRead as type of data
pub trait FromContent: Sized {
    const CONTENT_TYPE: &'static str = content_type::APPLICATION_JSON;
    const CHARSET: &'static str = content_type::CHARSET_UTF8;
    type Err;
    fn from_content(data: &[u8]) -> Result<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
pub trait IntoContent {
    const CONTENT_TYPE: &'static str = content_type::APPLICATION_JSON;
    const CHARSET: &'static str = content_type::CHARSET_UTF8;
    fn into_content(self) -> Vec<u8>;
}

pub mod content_type;
