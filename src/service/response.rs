use crate::cookie;
use crate::Result;
use crate::{http, RequestFail};
use cookie::Cookie;
use std::ops::Deref;
use std::collections::HashMap;

pub struct Response<T>(http::Response<T>);

impl<T> Deref for Response<T> {
    type Target = http::Response<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<http::Response<T>> for Response<T> {
    fn from(resp: http::Response<T>) -> Self {
        Self(resp)
    }
}

impl<T> Response<T> {
    pub fn cookies(&self) -> Result<Vec<Cookie>> {
        let mut cookies = Vec::new();
        for cookie in self.headers().get_all(http::header::SET_COOKIE) {
            cookies.push(Cookie::parse(cookie.to_str()?)?)
        }
        Ok(cookies)
    }

    pub fn cookie_map(&self) -> Result<HashMap<String, Cookie>> {
        Ok(self
            .cookies()?
            .into_iter()
            .map(|cookie| (cookie.name().to_owned(), cookie))
            .collect()
        )
    }
}

impl From<http::header::ToStrError> for RequestFail {
    fn from(err: http::header::ToStrError) -> Self {
        RequestFail::custom(err)
    }
}

impl From<cookie::ParseError> for RequestFail {
    fn from(err: cookie::ParseError) -> Self {
        RequestFail::custom(err)
    }
}
