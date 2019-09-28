use crate::{
    http::request::Builder as RequestBuilder,
    http::HeaderValue,
    mime::Mime,
    url::{ParseError, Url},
};

/// Client helper.
///
/// ### A default `Helper`
///
/// ```rust
/// use interfacer_http::Helper;
/// let helper = Helper::new();
/// ```
///
/// ### Custom `Helper`
///
/// ```rust
/// use interfacer_http::Helper;
/// use interfacer_http::http::{Version, header::USER_AGENT, request::Builder as RequestBuilder};
/// let helper = Helper::new()
///     .with_request_initializer(|| {
///         let mut builder = RequestBuilder::new();
///         builder
///             .version(Version::HTTP_10)
///             .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36");
///         builder
///     });
/// ```
#[derive(Clone)]
pub struct Helper {
    pub base_url: Option<Url>,
    pub request_initializer: fn() -> RequestBuilder,
    pub mime_matcher: fn(&Mime, &HeaderValue) -> bool,
}

impl Helper {
    pub fn new() -> Self {
        Self {
            base_url: None,
            request_initializer: RequestBuilder::new,
            mime_matcher: |expect, actual| match actual.to_str() {
                Ok(value) => expect == &value,
                Err(_) => false,
            },
        }
    }
}

impl Default for Helper {
    fn default() -> Self {
        Self::new()
    }
}

impl Helper {
    pub fn with_base_url(self, base_url: Url) -> Self {
        Self {
            base_url: Some(base_url),
            ..self
        }
    }

    pub fn with_request_initializer(self, request_initializer: fn() -> RequestBuilder) -> Self {
        Self {
            request_initializer,
            ..self
        }
    }

    pub fn with_mime_matcher(self, mime_matcher: fn(&Mime, &HeaderValue) -> bool) -> Self {
        Self {
            mime_matcher,
            ..self
        }
    }

    pub fn parse_uri(&self, raw_url: &str) -> Result<Url, ParseError> {
        match self.base_url {
            Some(ref base_url) => base_url.join(raw_url),
            None => raw_url.parse(),
        }
    }

    pub fn request(&self) -> RequestBuilder {
        (self.request_initializer)()
    }

    pub fn match_mime(&self, expect: &Mime, actual: &HeaderValue) -> bool {
        (self.mime_matcher)(expect, actual)
    }
}

#[cfg(test)]
mod tests {
    use super::{Helper, ParseError, RequestBuilder};
    use crate::http::{header::USER_AGENT, Error, Version};

    #[test]
    fn test_with_request_initializer() -> Result<(), Error> {
        let helper = Helper::new()
            .with_request_initializer(|| {
                let mut builder = RequestBuilder::new();
                builder
                    .version(Version::HTTP_10)
                    .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36");
                builder
            });
        let request = (helper.request_initializer)()
            .method("get")
            .uri("https://github.com")
            .body(())?;
        Ok(*request.body())
    }

    #[test]
    fn with_base_url() -> Result<(), ParseError> {
        let helper = Helper::new().with_base_url("https://github.com".parse()?);
        assert_eq!(
            helper.parse_uri("path")?.as_str(),
            "https://github.com/path"
        );
        Ok(())
    }
}
