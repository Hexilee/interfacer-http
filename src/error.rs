use crate::{
    http::{self, Response},
    url, FromContentError, ToContentError,
};
use std::fmt::{Debug, Display};
pub trait Error = From<url::ParseError>
    + From<http::Error>
    + From<ToContentError>
    + From<FromContentError>
    + From<Unexpected>
    + Display
    + Debug;

pub enum Unexpected {
    UnexpectedContentType(Response<Vec<u8>>),
    UnexpectedStatusCode(Response<Vec<u8>>),
}
