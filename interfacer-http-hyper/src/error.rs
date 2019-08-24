use derive_more::{Display, From};
use http::Response;
use interfacer_http::{http, url, FromContentError, ToContentError, Unexpected};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, From)]
pub enum Error {
    #[display(fmt = "hyper error: {}", _0)]
    Hyper(hyper::Error),

    #[display(fmt = "url parse error: {}", _0)]
    UrlParseError(url::ParseError),

    #[display(fmt = "http error: {}", _0)]
    HttpError(http::Error),

    #[display(fmt = "to content error: {}", _0)]
    ToContentError(ToContentError),

    #[display(fmt = "from content error: {}", _0)]
    FromContentError(FromContentError),

    #[display(fmt = "unexpected content type")]
    UnexpectedContentType(Response<Vec<u8>>),

    #[display(fmt = "unexpected status code")]
    UnexpectedStatusCode(Response<Vec<u8>>),
}

impl From<Unexpected> for Error {
    fn from(err: Unexpected) -> Self {
        match err {
            Unexpected::UnexpectedContentType(resp) => Error::UnexpectedContentType(resp),
            Unexpected::UnexpectedStatusCode(resp) => Error::UnexpectedStatusCode(resp),
        }
    }
}

impl std::error::Error for Error {}
