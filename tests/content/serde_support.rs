use crate::define_mirror_test;
use interfacer_http::content_types::{
    APPLICATION_FORM, APPLICATION_JSON, APPLICATION_MSGPACK, APPLICATION_XML, TEXT_XML,
};

use interfacer_http::derive::{FromContent, ToContent};
use interfacer_http::{ContentInto, ToContent};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromContent, ToContent, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}

macro_rules! define_test {
    ($base_type:expr, $encoding:expr) => {
        let user = User {
            name: "hexi".to_owned(),
            age: 18,
        };
        define_mirror_test!(user, $base_type, $encoding);
    };
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
#[test]
fn json() {
    define_test!(APPLICATION_JSON, None);
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
#[test]
fn application_xml() {
    define_test!(APPLICATION_XML, None);
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
#[test]
fn text_xml() {
    define_test!(TEXT_XML, None);
}

#[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
#[test]
fn urlencoded() {
    define_test!(APPLICATION_FORM, None);
}

#[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
#[test]
fn msgpack() {
    define_test!(APPLICATION_MSGPACK, None);
}

#[cfg(all(feature = "serde-full", feature = "encoding"))]
#[test]
fn encoding() {
    define_test!(APPLICATION_JSON, Some("utf-8"));
    define_test!(APPLICATION_FORM, Some("utf8"));
    define_test!(APPLICATION_XML, Some("gbk"));
    define_test!(TEXT_XML, Some("gb2312"));
}
