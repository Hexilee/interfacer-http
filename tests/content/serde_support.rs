use interfacer_http::mime::{
    APPLICATION_JSON, APPLICATION_MSGPACK, APPLICATION_WWW_FORM_URLENCODED, TEXT_XML,
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
        let mirror = $object
            .to_content(&content_type)?
            .content_into(&content_type)?;
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
fn json() -> Result<(), Box<dyn std::error::Error>> {
    define_test!(APPLICATION_JSON);
    Ok(())
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
#[test]
fn text_xml() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        name: "hexi".to_owned(),
        age: 18,
    };
    let mirror = user.to_content(&TEXT_XML)?.content_into(&TEXT_XML)?;
    assert_eq!(user, mirror);
    Ok(())
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
#[test]
fn application_xml() -> Result<(), Box<dyn std::error::Error>> {
    define_test!("application/xml".parse()?);
    Ok(())
}

#[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
#[test]
fn urlencoded() -> Result<(), Box<dyn std::error::Error>> {
    define_test!(APPLICATION_WWW_FORM_URLENCODED);
    Ok(())
}

#[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
#[test]
fn msgpack() -> Result<(), Box<dyn std::error::Error>> {
    define_test!(APPLICATION_MSGPACK);
    Ok(())
}

#[cfg(all(feature = "serde-full", feature = "encoding"))]
#[test]
fn encoding() -> Result<(), Box<dyn std::error::Error>> {
    define_test!("application/json; charset=utf-8".parse()?);
    define_test!("application/x-www-form-urlencoded; charset=gbk".parse()?);
    define_test!("text/xml; charset=gb2312".parse()?);
    Ok(())
}
