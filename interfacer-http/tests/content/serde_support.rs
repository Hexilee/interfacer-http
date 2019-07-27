#![cfg(feature = "derive")]

use interfacer_http::{FromContent, ToContent};
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
    let data =
        <User as ToContent<"application/json">>::to_content(&user, None).expect("to json fail");
    //    let mirror = <User as FromContent<"application/json">>::from_content(&data, None)
    //        .expect("from json fail");
    //    assert_eq!(user, mirror);
}
