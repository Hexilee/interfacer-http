use crate::{
    http::request::Builder,
    url::{ParseError, Url},
};
use std::sync::Arc;

pub trait UrlParser = Sync + Send + Fn(&str) -> Result<Url, ParseError>;

pub type RequestInitializer = fn(&mut Builder) -> &mut Builder;

#[derive(Clone)]
pub struct HttpConfig {
    pub url_parser: Arc<dyn UrlParser>,
    pub request_initializer: RequestInitializer,
}

impl HttpConfig {
    pub fn new() -> Self {
        Self {
            url_parser: Arc::new(|raw_url| raw_url.parse()),
            request_initializer: |builder| builder,
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

    pub fn with_request_initializer(self, initializer: RequestInitializer) -> Self {
        Self {
            request_initializer: initializer,
            ..self
        }
    }
}

pub fn base_on(base_url: Url) -> impl Fn(&str) -> Result<Url, ParseError> {
    move |path| base_url.join(path)
}

#[cfg(test)]
mod tests {
    use super::{base_on, Builder, HttpConfig, ParseError};
    use crate::http::{header::USER_AGENT, Error, Request, Version};
    use crate::url::Url;

    #[test]
    fn test_with_request_initializer() -> Result<(), Error> {
        let config = HttpConfig::new()
            .with_request_initializer(|builder: &mut Builder|
                builder
                    .version(Version::HTTP_10)
                    .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36")
            );
        let request =
            (config.request_initializer)(&mut Request::get("https://github.com")).body(())?;
        Ok(*request.body())
    }

    #[test]
    fn test_with_url_parser() -> Result<(), ParseError> {
        let config = HttpConfig::new().with_url_parser((|base_uri: Url| {
            move |path: &str| base_uri.join(path)
        })("https://github.com".parse()?));
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
