#![cfg(feature = "derive")]

use interfacer_http::{ContentType, FromContent, ToContent};
use interfacer_http_util::content_types::APPLICATION_JSON;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
#[test]
fn json() {
    let user = User {
        name: "hexi".to_owned(),
        age: 18,
    };
    let content_type = ContentType::new(APPLICATION_JSON, None);
    let data = <User as ToContent>::to_content(&user, &content_type).expect("to json fail");
    let mirror = <User as FromContent>::from_content(data, &content_type).expect("from json fail");
    assert_eq!(user, mirror);
}
