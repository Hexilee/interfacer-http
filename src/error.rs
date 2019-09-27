use crate::{
    http::{self, header::HeaderName, Response, StatusCode},
    url, FromContentError, ToContentError,
};
use derive_more::{Constructor, Display, From};
use std::fmt::{Debug, Display};

/// Error trait to constrain `HttpClient::Err`.
pub trait Error = From<url::ParseError>
    + From<http::Error>
    + From<ToContentError>
    + From<FromContentError>
    + From<Unexpected>
    + Display
    + Debug;

/// Error for `Response` asserting.
#[derive(Debug, Display, Constructor)]
#[display(fmt = "Unexpected: {}", typ)]
pub struct Unexpected {
    typ: UnexpectedType,
    resp: Response<Vec<u8>>,
}

/// Error type for `Response` asserting.
#[derive(Debug, Display, From)]
pub enum UnexpectedType {
    #[display(fmt = "status code should be {}", expect)]
    StatusCode { expect: StatusCode },
    #[display(fmt = "value of header '{}' is unexpected: {}", header_name, msg)]
    Header {
        header_name: HeaderName,
        msg: String,
    },
}

impl std::error::Error for Unexpected {}
impl std::error::Error for UnexpectedType {}
