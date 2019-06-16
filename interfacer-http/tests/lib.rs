#![cfg(test)]
use interfacer_http::http_service;

mod basic;

#[http_service]
trait BasicService {}
