#![cfg(test)]

use interfacer::http::HttpClient;
use interfacer_http::{get, http_service};

struct User {}

#[http_service]
trait BasicService {
    #[get(
        path = "/api/user/{id}",
        expect(status = 200, content_type = "application/json")
    )]
    fn get_user(&self, id: u64);
}

mod basic;
