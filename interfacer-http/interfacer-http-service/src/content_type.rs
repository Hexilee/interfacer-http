use crate::{fail::StringError, RequestFail, Result};

#[derive(Eq, PartialEq)]
pub struct ContentType {
    base_type: String,
    charset: Option<String>,
}

impl ContentType {
    // parse Response Content-Type
    pub fn from_raw(raw: impl AsRef<str>) -> Result<Self> {
        let segments = raw
            .as_ref()
            .split(';')
            .map(|segment: &str| segment.trim())
            .collect::<Vec<&str>>();
        match segments.len() {
            1 => Ok(Self {
                base_type: segments[0].into(),
                charset: None,
            }),
            2 => Ok(Self {
                base_type: segments[0].into(),
                charset: Some(segments[1].into()),
            }),
            _ => Err(RequestFail::http(StringError::new(format!(
                "Content-Type({}) of response parse fail",
                raw.as_ref()
            )))),
        }
    }

    pub fn new(base_type: &str, charset: Option<&str>) -> Self {
        Self {
            base_type: base_type.into(),
            charset: charset.map(|refer| refer.into()),
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
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match &self.charset {
            Some(charset) => format!("{}; {}", &self.base_type, charset),
            None => self.base_type.clone(),
        }
    }
}

// charset
pub const CHARSET_UTF8: &str = "utf-8";
pub const CHARSET_GB2312: &str = "gb2312";
pub const CHARSET_GBK: &str = "gbk";

// content-type
pub const APPLICATION_JSON: &str = "application/json";
pub const APPLICATION_JAVA_SCRIPT: &str = "application/javascript";
pub const APPLICATION_XML: &str = "application/xml";
pub const TEXT_XML: &str = "text/xml";
pub const APPLICATION_FORM: &str = "application/x-www-form-urlencoded";
pub const APPLICATION_PROTOBUF: &str = "application/protobuf";
pub const APPLICATION_MSGPACK: &str = "application/msgpack";
pub const TEXT_HTML: &str = "text/html";
pub const TEXT_PLAIN: &str = "text/plain";
pub const MULTIPART_FORM: &str = "multipart/form-data";
pub const OCTET_STREAM: &str = "application/octet-stream";
