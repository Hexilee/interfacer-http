#![feature(custom_attribute, async_await)]
#![cfg(test)]

use interfacer_http::{http_service, Result};

struct User {}

#[http_service]
trait BasicService: Clone {
    #[get(path = "/api/user/{id}")]
    #[expect(status = 200, content_type = "application/json")]
    async fn get_user(&self, id: u64) -> Result<User> {}
}

mod basic;
