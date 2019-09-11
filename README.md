A magic http client, like retrofit in Java

[![Build status](https://img.shields.io/travis/Hexilee/interfacer-http/master.svg)](https://travis-ci.org/Hexilee/interfacer-http)
[![Coverage Status](https://coveralls.io/repos/github/Hexilee/interfacer-http/badge.svg?branch=master)](https://coveralls.io/github/Hexilee/interfacer-http?branch=master)
[![Crate version](https://img.shields.io/crates/v/interfacer-http.svg)](https://crates.io/crates/interfacer-http)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/interfacer-http/blob/master/LICENSE)
[![Rust Docs](https://docs.rs/interfacer-http/badge.svg)](https://docs.rs/interfacer-http)

```rust
// define interface

#![feature(custom_attribute, async_await, param_attrs)]
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

// use it
use interfacer_http_hyper::AsyncService;
```