use super::User;
use interfacer_http_util::content_types::{
    APPLICATION_FORM, APPLICATION_JSON, APPLICATION_MSGPACK, APPLICATION_XML, TEXT_XML,
};

macro_rules! define_test {
    ($base_type:expr, $encoding:expr) => {
        let user = User {
            name: "hexi".to_owned(),
            age: 18,
        };
        let content_type = interfacer_http::ContentType::new($base_type, $encoding);
        let data = <User as interfacer_http::ToContent>::to_content(&user, &content_type)
            .expect(&format!("to '{}' fail", $base_type));
        let mirror = <User as interfacer_http::FromContent>::from_content(data, &content_type)
            .expect(&format!("from '{}' fail", $base_type));
        assert_eq!(user, mirror);
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

#[cfg(any(feature = "serde-full", feature = "encode"))]
#[test]
fn encoding() {
    define_test!(APPLICATION_JSON, Some("utf-8"));
    define_test!(APPLICATION_XML, Some("gbk"));
    define_test!(TEXT_XML, Some("gb2312"));
}
