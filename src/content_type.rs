use encoding::{all::UTF_8, EncodingRef};
use mime::{Mime, CHARSET};

pub struct ApplicationJSON;

impl MimeType for ApplicationJSON {
    const TYPE: &'static str = "application/json";
    const ENCODING: Option<EncodingRef> = Some(UTF_8);
    const ENCODING_LABELS: Option<&'static [&'static str]> = None;
}

pub trait MimeType {
    const TYPE: &'static str;
    const ENCODING: Option<EncodingRef>;
    const ENCODING_LABELS: Option<&'static [&'static str]>;
    fn equal(content_type: &Mime) -> bool {
        if content_type.pure_type() != Self::TYPE {
            return false;
        }

        match Self::ENCODING_LABELS {
            Some(labels) => match content_type.get_param(CHARSET) {
                None => false,
                Some(charset) if !labels.contains(&charset.as_str()) => false,
                _ => true,
            },
            None => true,
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
    use super::{ApplicationJSON, Mime, MimeExt, MimeType};
    use mime::APPLICATION_JSON;
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

    #[test]
    fn test_basic_mime_equal() {
        assert!(ApplicationJSON::equal(&APPLICATION_JSON));
    }
}
