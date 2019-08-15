#![feature(custom_attribute, async_await, param_attrs)]
#![cfg(all(feature = "derive", feature = "serde-full"))]
#![allow(unused_attributes)]

use interfacer_http::derive::{FromContent, ToContent};
use interfacer_http::{content_types, http::header::COOKIE, http_interface, Response, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromContent, ToContent, Debug)]
struct User {
    name: String,
    age: i32,
}

#[http_interface]
trait UserInterface: Clone {
    #[put("/api/user/{id}?age={age}")]
    #[expect(200, content_types::APPLICATION_JSON)]
    async fn put_user(
        &self,
        id: u64,
        age: i32,
        #[body] user: &User,
        #[header(COOKIE)] cookie: &str
    ) -> Result<Response<User>>;
}
