#![cfg(feature = "derive")]
#![feature(custom_attribute, async_await)]
#![allow(unused_attributes)]

use interfacer_http::derive::{FromContent, ToContent};
use interfacer_http::{http_service, Response, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromContent, ToContent, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}

#[http_service]
trait UserService: Clone {
    #[get("/api/user/{id}?age={age}")]
    #[expect(200, "application/json")]
    async fn put_user(&self, id: u64, age: i32, user: &User) -> Result<Response<User>> {}
}
