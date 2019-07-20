#![cfg(test)]
use interfacer::http::HttpClient;
use interfacer_http::{get, http_service};

#[http_service]
trait BasicService {
    #[get("/api/user/{id}", expect(200, APPLICATION_JSON))]
    fn get_user(&self, id: u64);
}

mod basic;
