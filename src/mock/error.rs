use crate::{http, url, FromContentError, ToContentError, Unexpected};
use derive_more::{Display, From};

#[derive(Display, Debug, From)]
pub enum Error {
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

pub type Result<T> = std::result::Result<T, Error>;
