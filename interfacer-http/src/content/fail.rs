use crate::RequestFail;
use failure::Fail;
use std::fmt::Display;

#[derive(Debug)]
struct EncodeFail(Box<dyn Fail>);

impl Display for EncodeFail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for EncodeFail {}

impl From<EncodeFail> for RequestFail {
    fn from(err: EncodeFail) -> Self {
        RequestFail::custom(err)
    }
}
