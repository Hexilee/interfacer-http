use failure::Fail;
use http::StatusCode;
use std::error::Error;
use std::fmt::Display;
use url::ParseError;

pub type Result<T> = std::result::Result<T, RequestFail>;

#[derive(Fail, Debug)]
pub enum RequestFail {
    #[fail(display = "url parse fail: {}", err)]
    UrlParse { err: ParseError },

    #[fail(display = "request build fail: {}", err)]
    RequestBuild { err: http::Error },

    #[fail(display = "http fail: {}", err)]
    HTTP { err: Box<dyn Fail> },

    #[fail(display = "encode into content fail: {}", err)]
    Encode { err: Box<dyn Fail> },

    #[fail(display = "unexpected status code: {}", code)]
    StatusCode { code: StatusCode },

    #[fail(display = "unexpected content type: {}", content_type)]
    ContentType { content_type: String },

    #[fail(display = "decode from content fail: {}", err)]
    Decode { err: Box<dyn Fail> },
}

#[derive(Debug)]
pub struct StringError {
    display: String,
}

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.display.as_str())
    }
}

impl Error for StringError {}

impl StringError {
    pub fn new(display: impl Into<String>) -> Self {
        Self {
            display: display.into(),
        }
    }
}

impl RequestFail {
    pub fn http(err: impl Fail) -> Self {
        RequestFail::HTTP { err: Box::new(err) }
    }

    pub fn encode(err: impl Fail) -> Self {
        RequestFail::Encode { err: Box::new(err) }
    }

    pub fn decode(err: impl Fail) -> Self {
        RequestFail::Decode { err: Box::new(err) }
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
