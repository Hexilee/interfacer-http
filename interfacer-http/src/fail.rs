use crate::http::{self, StatusCode};
use crate::url::ParseError;
use failure::Fail;
use std::error::Error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, RequestFail>;

#[derive(Fail, Debug)]
pub enum RequestFail {
    #[fail(display = "url parse fail: {}", err)]
    UrlParse { err: ParseError },

    #[fail(display = "request build fail: {}", err)]
    RequestBuild { err: http::Error },

    #[fail(display = "unexpected status code: {}", code)]
    StatusCode { code: StatusCode },

    #[fail(display = "unexpected content type: {}", content_type)]
    ContentType { content_type: String },

    #[fail(display = "{}", err)]
    Custom { err: Box<dyn Fail> },
}

#[derive(Debug)]
pub struct StringError(String);

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Error for StringError {}

impl StringError {
    pub fn new(display: impl Into<String>) -> Self {
        Self(display.into())
    }
}

impl RequestFail {
    pub fn custom(err: impl Fail) -> Self {
        RequestFail::Custom { err: Box::new(err) }
    }

    pub fn expect_status(expect_status: StatusCode, ret_status: StatusCode) -> Result<()> {
        if expect_status == ret_status {
            Ok(())
        } else {
            Err(RequestFail::StatusCode { code: ret_status })
        }
    }
}

impl From<ParseError> for RequestFail {
    fn from(err: ParseError) -> Self {
        RequestFail::UrlParse { err }
    }
}

impl From<http::Error> for RequestFail {
    fn from(err: http::Error) -> Self {
        RequestFail::RequestBuild { err }
    }
}

pub macro define_from($from:ty) {
    impl From<$from> for RequestFail {
        fn from(err: $from) -> Self {
            RequestFail::custom(err)
        }
    }
}
