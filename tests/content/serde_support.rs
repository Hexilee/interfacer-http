use interfacer_http::mime::{
    FromStrError, APPLICATION_JSON, APPLICATION_MSGPACK, APPLICATION_WWW_FORM_URLENCODED, TEXT_XML,
};

use interfacer_http::{ContentInto, ToContent};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}

macro_rules! define_mirror_test {
    ($object:expr, $content_type:expr) => {
        let content_type: interfacer_http::mime::Mime = $content_type;
        let data = $object
            .to_content(&content_type)
            .expect(&format!("to '{}' fail", content_type.as_ref()));
        let mirror = data
            .content_into(&content_type)
            .expect(&format!("from '{}' fail", content_type.as_ref()));
        assert_eq!($object, mirror);
    };
}

macro_rules! define_test {
    ($content_type:expr) => {
        let user = User {
            name: "hexi".to_owned(),
            age: 18,
        };
        define_mirror_test!(user, $content_type);
    };
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
#[test]
fn json() {
    define_test!(APPLICATION_JSON);
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
#[test]
fn text_xml() {
    define_test!(TEXT_XML);
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
#[test]
fn application_xml() -> Result<(), FromStrError> {
    define_test!("application/xml".parse()?);
    Ok(())
}

#[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
#[test]
fn urlencoded() {
    define_test!(APPLICATION_WWW_FORM_URLENCODED);
}

#[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
#[test]
fn msgpack() {
    define_test!(APPLICATION_MSGPACK);
}

#[cfg(all(feature = "serde-full", feature = "encoding"))]
#[test]
fn encoding() -> Result<(), FromStrError> {
    define_test!("application/json; charset=utf-8".parse()?);
    define_test!("application/www-urlencoded-form; charset=gbk".parse()?);
    define_test!("text/xml; charset=gb2312".parse()?);
    Ok(())
}
