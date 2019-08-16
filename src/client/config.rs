use crate::{
    http::request::Builder,
    url::{ParseError, Url},
};
use std::sync::Arc;

pub trait UrlParser = Sync + Send + Fn(&str) -> Result<Url, ParseError>;

pub trait RequestInitializer = Sync + Send + Fn(&mut Builder) -> &mut Builder;

#[derive(Clone)]
pub struct HttpConfig {
    pub url_parser: Arc<dyn UrlParser>,
    pub request_initializer: Arc<dyn RequestInitializer>,
}

impl HttpConfig {
    pub fn new() -> Self {
        Self {
            url_parser: Arc::new(|raw_url| raw_url.parse()),
            request_initializer: Arc::new(|builder| builder),
        }
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpConfig {
    pub fn with_url_parser(self, parser: impl 'static + UrlParser) -> Self {
        Self {
            url_parser: Arc::new(parser),
            ..self
        }
    }

    pub fn with_request_initializer(self, initializer: impl 'static + RequestInitializer) -> Self {
        Self {
            request_initializer: Arc::new(initializer),
            ..self
        }
    }
}

pub fn base_on(base_url: Url) -> impl Fn(&str) -> Result<Url, ParseError> {
    move |path| base_url.join(path)
}

#[cfg(test)]
mod tests {
    use super::{base_on, HttpConfig, ParseError};
    use interfacer_http_util::url::Url;

    #[test]
    fn test_with_url_parser() -> Result<(), ParseError> {
        let config =
            HttpConfig::new().with_url_parser((|base_uri: Url| move |path| base_uri.join(path))(
                "https://github.com".parse()?,
            ));
        assert_eq!(
            (*config.url_parser)("path")?.as_str(),
            "https://github.com/path"
        );
        Ok(())
    }

    #[test]
    fn test_base_on() -> Result<(), ParseError> {
        let config = HttpConfig::new().with_url_parser(base_on("https://github.com".parse()?));
        assert_eq!(
            (*config.url_parser)("path")?.as_str(),
            "https://github.com/path"
        );
        Ok(())
    }
}
