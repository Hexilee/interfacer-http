use crate::{fail::StringError, url::form_urlencoded, RequestFail, Result};

const CHARSET: &'static str = "charset";
const BOUNDARY: &'static str = "boundary";

#[derive(Clone)]
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
            0 => Err(RequestFail::custom(StringError::new(
                "Content-Type({}) is empty",
            ))),
            n => {
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
