pub use http::{
    header, header::HeaderMap, method, method::Method, request, request::Request, response,
    response::Response, status, status::StatusCode, version, version::Version, HttpTryFrom,
};

pub use url::Url;

pub use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub use async_trait::async_trait;

pub use fail::{RequestFail, Result};

pub mod content_type;
use core::result::Result as StdResult;

// TODO: use T: AsyncRead as type of Request::Body
// TODO: use T: AsyncRead as type of Response::Body
#[async_trait]
pub trait HttpClient {
    type Err: Into<RequestFail>;
    async fn request(&self, req: Request<Vec<u8>>) -> StdResult<Response<Vec<u8>>, Self::Err>;
}

pub trait HttpService {
    type Client: HttpClient;
    fn get_base_url(&self) -> &Url;
    fn get_client(&self) -> &Self::Client;
}

// TODO: use T: AsyncRead as type of data
pub trait FromContent: Sized {
    const CONTENT_TYPE: &'static str = content_type::APPLICATION_JSON;
    const CHARSET: &'static str = content_type::CHARSET_UTF8;
    type Err;
    fn from_content(data: &[u8]) -> StdResult<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
pub trait IntoContent {
    const CONTENT_TYPE: &'static str = content_type::APPLICATION_JSON;
    const CHARSET: &'static str = content_type::CHARSET_UTF8;
    fn into_content(self) -> Vec<u8>;
}

mod fail;
