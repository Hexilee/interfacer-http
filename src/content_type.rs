use encoding::{all::UTF_8, label::encoding_from_whatwg_label, EncodingRef};
use mime::{Mime, CHARSET};

pub struct Encoding(Option<EncodingRef>, Option<&'static [&'static str]>);
const ENCODING_DEFAULT: Encoding = Encoding(Some(UTF_8), None);
const UTF8: Encoding = Encoding(Some(UTF_8), Some(&["unicode-1-1-utf-8", "utf8", "utf-8"]));

macro_rules! define_mime_type {
    ($name:ident, $typ:expr, $encoding:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $name;
        impl MimeType for $name {
            const TYPE: &'static str = $typ;
            const ENCODING: Encoding = $encoding;
        }
    };
}

macro_rules! define_mime_types {
    ($(($name:ident, $typ:expr, $encoding:expr)),*) => {
        $(define_mime_type!($name, $typ, $encoding);)*
    };
}

define_mime_types!(
    (APPLICATION_JSON, "application/json", ENCODING_DEFAULT),
    (APPLICATION_JSON_UTF8, "application/json", UTF8)
);

pub trait MimeType {
    const TYPE: &'static str;
    const ENCODING: Encoding;
    fn equal(content_type: &Mime) -> bool {
        if content_type.pure_type() != Self::TYPE {
            return false;
        }

        match Self::ENCODING.1 {
            Some(labels) => match content_type.get_param(CHARSET) {
                None => false,
                Some(charset) if !labels.contains(&charset.as_str().to_lowercase().as_str()) => {
                    false
                }
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
    use super::{Mime, MimeExt, MimeType, APPLICATION_JSON};
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
        assert!(APPLICATION_JSON::equal(&mime::APPLICATION_JSON));
    }
}
