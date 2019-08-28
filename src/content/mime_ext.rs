use crate::mime::Mime;

pub trait MimeExt {
    fn pure_type(&self) -> Self;
}

impl MimeExt for Mime {
    fn pure_type(&self) -> Self {
        let mut ret = format!("{}/{}", self.type_(), self.subtype());
        match self.suffix() {
            Some(suffix) => {
                ret += "+";
                ret += suffix.as_str();
            }
            None => (),
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
