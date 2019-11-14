use encoding::{all::UTF_8, EncodingRef};
use mime::{Mime, CHARSET};
pub trait MimeType {
    const TYPE: &'static str = "application/json";
    const ENCODING: EncodingRef = UTF_8;
    fn equal(content_type: &Mime) -> bool {
        match Self::TYPE.parse::<Mime>() {
            Err(_) => false,
            Ok(self_type) => {
                if self_type.pure_type() != content_type.pure_type() {
                    return false;
                }

                if self_type.get_param(CHARSET).is_some()
                    && content_type.get_param(CHARSET).is_some()
                {
                    return self_type.get_param(CHARSET) == content_type.get_param(CHARSET);
                }

                true
            }
        }
    }
}

/// Extensional trait for `mime::Mime`.
pub trait MimeExt {
    fn pure_type(&self) -> Self;
}

impl MimeExt for Mime {
    fn pure_type(&self) -> Self {
        let mut ret = format!("{}/{}", self.type_(), self.subtype());
        if let Some(suffix) = self.suffix() {
            ret += "+";
            ret += suffix.as_str();
        }
        ret.parse().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::{Mime, MimeExt};
    use std::str::FromStr;
    #[test]
    fn test_pure_type() {
        assert_eq!(
            &Mime::from_str("application/json; charset=utf-8; FOO=BAR")
                .unwrap()
                .pure_type(),
            &"application/json"
        );
        assert_eq!(
            &Mime::from_str("image/svg+xml; FOO=BAR")
                .unwrap()
                .pure_type(),
            &"image/svg+xml"
        );
    }
}
