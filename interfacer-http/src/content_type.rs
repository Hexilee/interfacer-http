use crate::{fail::StringError, RequestFail, Result};

// TODO: support boundary
#[derive(Eq, PartialEq, Clone)]
pub struct ContentType {
    base_type: String,
    encoding: Option<String>,
    //    boundary: Option<String>,
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
            1 => Ok(Self {
                base_type: segments[0].into(),
                encoding: None,
            }),
            2 => Ok(Self {
                base_type: segments[0].into(),
                encoding: Some(segments[1].into()),
            }),
            _ => Err(RequestFail::custom(StringError::new(format!(
                "Content-Type({}) of parse fail",
                raw.as_ref()
            )))),
        }
    }

    pub fn new(base_type: &str, encoding: Option<&str>) -> Self {
        Self {
            base_type: base_type.into(),
            encoding: encoding.map(|refer| refer.into()),
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
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self.encoding() {
            Some(encoding) => format!("{}; {}", self.base_type(), encoding),
            None => self.base_type().to_owned(),
        }
    }
}
