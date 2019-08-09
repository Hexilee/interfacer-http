use crate::{http::HeaderValue, url::form_urlencoded, RequestFail, Result, StringError};

const CHARSET: &str = "charset";
const BOUNDARY: &str = "boundary";

#[derive(Clone, Debug)]
pub struct ContentType {
    base_type: String,
    encoding: Option<String>,
    boundary: Option<String>,
}

impl ContentType {
    // parse Content-Type
    pub fn from_raw(raw: impl AsRef<str>) -> Result<Self> {
        let segments = raw
            .as_ref()
            .split(';')
            .map(|segment: &str| segment.trim())
            .collect::<Vec<&str>>();
        match segments.len() {
            0 => Err(StringError::new(
                "Content-Type({}) is empty",
            ).into()),
            n => {
                if segments[0] == "" {
                    return Err(StringError::new(
                        "base type of Content-Type({}) is empty",
                    ).into());
                }

                let mut ret = Self {
                    base_type: segments[0].into(),
                    encoding: None,
                    boundary: None,
                };
                if n > 1 {
                    let info: String = segments[1..].join("&");
                    let pairs = form_urlencoded::parse(info.as_bytes());
                    for (key, value) in pairs {
                        match &*key {
                            CHARSET => {
                                ret.encoding = Some(value.into_owned());
                            }
                            BOUNDARY => {
                                ret.boundary = Some(value.into_owned());
                            }
                            _ => (),
                        }
                    }
                }
                Ok(ret)
            }
        }
    }

    pub fn from_header(header: &HeaderValue) -> Result<Self> {
        let header_raw = header
            .to_str()
            .map_err(|err| StringError::new(format!("HeaderValue cannot to str: {}", err)))?;
        Self::from_raw(header_raw)
    }

    pub fn new(base_type: &str, encoding: Option<&str>, boundary: Option<&str>) -> Self {
        Self {
            base_type: base_type.into(),
            encoding: encoding.map(|refer| refer.into()),
            boundary: boundary.map(|refer| refer.into()),
        }
    }

    pub fn expect(&self, other: &Self) -> Result<()> {
        if self == other {
            Ok(())
        } else {
            Err(RequestFail::ContentType {
                content_type: other.to_string(),
            })
        }
    }

    pub fn base_type(&self) -> &str {
        &self.base_type
    }

    pub fn encoding(&self) -> Option<&str> {
        self.encoding.as_ref().map(|encoding| encoding.as_str())
    }

    pub fn boundary(&self) -> Option<&str> {
        self.boundary.as_ref().map(|boundary| boundary.as_str())
    }
}

impl PartialEq for ContentType {
    fn eq(&self, other: &Self) -> bool {
        self.base_type() == other.base_type() && self.encoding() == other.encoding()
    }
}

impl Eq for ContentType {}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        let mut list = vec![self.base_type().to_owned()];
        if let Some(encoding) = self.encoding() {
            list.push(format!("{}={}", CHARSET, encoding));
        }
        if let Some(boundary) = self.boundary() {
            list.push(format!("{}={}", BOUNDARY, boundary));
        }
        list.join("; ")
    }
}

impl std::convert::TryFrom<&str> for ContentType {
    type Error = RequestFail;
    fn try_from(value: &str) -> Result<Self> {
        Self::from_raw(value)
    }
}

#[cfg(test)]
mod test {
    use super::ContentType;
    macro_rules! define_reverse_test {
        ($raw:expr) => {
            let content_type = ContentType::from_raw($raw).expect("parse Content-Type fail");
            assert_eq!($raw, &content_type.to_string());
        };
    }

    #[test]
    fn reverse() {
        define_reverse_test!("text/html");
        define_reverse_test!("text/html; charset=gb2312");
        define_reverse_test!("text/html; boundary=98665v78gh6r9g6trf6tg67stgft");
        define_reverse_test!("text/html; charset=utf-7; boundary=98665v78gh6r9g6trf6tg67stgft");
    }

    #[test]
    #[should_panic(expected = "parse Content-Type fail")]
    fn empty() {
        define_reverse_test!("");
    }

    #[test]
    #[should_panic(expected = "parse Content-Type fail")]
    fn base_type_empty() {
        define_reverse_test!(" ; charset=gbk");
    }

    macro_rules! define_equal_test {
        ($raw:expr, $base_type:expr, $encoding:expr) => {
            let result = ContentType::from_raw($raw).expect("parse Content-Type fail");
            assert_eq!(&ContentType::new($base_type, $encoding, None), &result);
        };
    }

    #[test]
    fn equal() {
        define_equal_test!("text/html", "text/html", None);
        define_equal_test!("text/html; charset=gb2312", "text/html", Some("gb2312"));
        define_equal_test!(
            "text/html; boundary=98665v78gh6r9g6trf6tg67stgft",
            "text/html",
            None
        );
        define_equal_test!(
            "text/html; charset=utf-7; boundary=98665v78gh6r9g6trf6tg67stgft",
            "text/html",
            Some("utf-7")
        );
    }

    #[test]
    #[should_panic]
    fn different_base_type() {
        define_equal_test!("text/html", "text/xml", None);
    }

    #[test]
    #[should_panic]
    fn different_encoding() {
        define_equal_test!("text/html; charset=gb2312", "text/html", Some("utf-7"));
    }
}
