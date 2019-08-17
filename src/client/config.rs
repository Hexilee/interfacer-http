use crate::{
    http::request::Builder as RequestBuilder,
    url::{ParseError, Url},
};

#[derive(Clone)]
pub struct Config {
    pub base_url: Option<Url>,
    pub request_initializer: fn() -> RequestBuilder,
}

impl Config {
    pub fn new() -> Self {
        Self {
            base_url: None,
            request_initializer: RequestBuilder::new,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
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

    pub fn parse_url(&self, raw_url: &str) -> Result<Url, ParseError> {
        match self.base_url {
            Some(ref base_url) => base_url.join(raw_url),
            None => raw_url.parse(),
        }
    }

    pub fn request(&self) -> RequestBuilder {
        (self.request_initializer)()
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ParseError, RequestBuilder};
    use crate::http::{header::USER_AGENT, Error, Version};

    #[test]
    fn test_with_request_initializer() -> Result<(), Error> {
        let config = Config::new()
            .with_request_initializer(|| {
                let mut builder = RequestBuilder::new();
                builder
                    .version(Version::HTTP_10)
                    .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36");
                builder
            });
        let request = (config.request_initializer)()
            .method("get")
            .uri("https://github.com")
            .body(())?;
        Ok(*request.body())
    }

    #[test]
    fn with_base_url() -> Result<(), ParseError> {
        let config = Config::new().with_base_url("https://github.com".parse()?);
        assert_eq!(
            config.parse_url("path")?.as_str(),
            "https://github.com/path"
        );
        Ok(())
    }
}
