#![cfg(test)]
use interfacer_http::{expect, get, http_service};

#[http_service]
trait BasicService {
    #[get("/api/user/{id}")]
    #[expect(200, APPLICATION_JSON)]
    fn get_user(&self, id: u64);
}

mod basic;
