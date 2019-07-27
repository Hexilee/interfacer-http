#![cfg(feature = "derive")]
#![feature(custom_attribute, async_await)]

use interfacer_http::derive::{FromContent, ToContent};
use interfacer_http::{http_service, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromContent, ToContent, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}

#[http_service]
trait UserService: Clone {
    #[get(path = "/api/user/{id}")]
    #[expect(status = 200, content_type = "application/json")]
    async fn put_user(&self, id: u64, user: &User) -> Result<User> {}
}
