use derive_more::{Display, From};
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

    #[display(fmt = "{}", _0)]
    Unexpected(Unexpected),
}

impl std::error::Error for Error {}
