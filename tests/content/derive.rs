use interfacer_http::{
    define_from_content, define_to_content,
    mime::{self, Mime},
    ContentInto, FromContent, FromContentError, ToContent, ToContentError,
};

use std::str::FromStr;
use std::string::ToString;

define_from_content!(FromContentString);
define_to_content!(StringToContent);

impl<T: FromStr> FromContentString for T
where
    T::Err: std::string::ToString,
{
    fn _from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, FromContentError> {
        String::from_utf8_lossy(&data)
            .parse()
            .map_err(|err: <Self as FromStr>::Err| {
                (data, content_type.clone(), err.to_string()).into()
            })
    }
}

impl<T: ToString> StringToContent for T {
    fn _to_content(&self, _content_type: &Mime) -> Result<Vec<u8>, ToContentError> {
        Ok(self.to_string().into_bytes())
    }
}

#[derive(Debug, Eq, PartialEq, FromContent, ToContent)]
struct I32(i32);

impl FromStr for I32 {
    type Err = <i32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl ToString for I32 {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[test]
fn test_to_content() {
    let data = I32(1).to_content(&mime::TEXT_PLAIN).unwrap();
    assert_eq!("1", String::from_utf8_lossy(&data).as_ref());
}

#[test]
fn test_from_content() {
    assert_eq!(
        I32(1),
        "1".to_owned()
            .into_bytes()
            .content_into(&mime::TEXT_PLAIN)
            .unwrap()
    );
}
