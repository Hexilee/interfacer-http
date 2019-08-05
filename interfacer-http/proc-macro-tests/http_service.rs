#![cfg(feature = "derive")]
#![feature(custom_attribute, async_await, param_attrs)]
#![allow(unused_attributes)]

use interfacer_http::derive::{FromContent, ToContent};
use interfacer_http::{content_types, http_service, Response, Result, http::header::COOKIE};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromContent, ToContent, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}

#[http_service]
trait UserService: Clone {
    #[put("/api/user/{id}?age={age}")]
    #[expect(200, content_types::APPLICATION_JSON)]
    async fn put_user(
        &self,
        id: u64,
        age: i32,
        #[body] user: &User,
        #[header(COOKIE)] cookie: &str
    ) -> Result<Response<User>> {}
}
