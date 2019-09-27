use crate::cookie;
use crate::http::{header, HeaderValue, Response};
use cookie::Cookie;
use derive_more::{Display, From};
use std::collections::HashMap;

/// Error for cookie string parsing.
#[derive(Debug, Display, From)]
pub enum CookieError {
    #[display(fmt = "cookie value (`{:?}`) is not string: {}", value, msg)]
    ValueNotStr {
        value: HeaderValue,
        msg: header::ToStrError,
    },

    #[display(fmt = "parse cookie('{}') error: {}", value, msg)]
    ParseError {
        value: String,
        msg: cookie::ParseError,
    },
}

/// Extensional trait for `http::Response`.
pub trait ResponseExt {
    fn cookies(&self) -> Result<Vec<Cookie>, CookieError>;
    fn cookie_map(&self) -> Result<HashMap<String, Vec<Cookie>>, CookieError>;
}

impl<T> ResponseExt for Response<T> {
    fn cookies(&self) -> Result<Vec<Cookie>, CookieError> {
        let mut cookies = Vec::new();
        for cookie in self.headers().get_all(header::SET_COOKIE) {
            let cookie_str = cookie.to_str().map_err(|err| (cookie.clone(), err))?;
            cookies.push(Cookie::parse(cookie_str).map_err(|err| (cookie_str.to_owned(), err))?)
        }
        Ok(cookies)
    }

    fn cookie_map(&self) -> Result<HashMap<String, Vec<Cookie>>, CookieError> {
        let mut map = HashMap::new();
        for cookie in self.headers().get_all(header::SET_COOKIE) {
            let cookie_str = cookie.to_str().map_err(|err| (cookie.clone(), err))?;
            let cookie = Cookie::parse(cookie_str).map_err(|err| (cookie_str.to_owned(), err))?;
            match map.get_mut(cookie.name()) {
                None => {
                    map.insert(cookie.name().to_owned(), vec![cookie]);
                }
                Some(list) => list.push(cookie),
            }
        }
        Ok(map)
    }
}

impl std::error::Error for CookieError {}
