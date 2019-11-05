#![feature(custom_attribute, param_attrs)]
#![allow(unused_attributes)]

use interfacer_http::{
    http::Response,
    http_service,
};

#[rustfmt::skip]
#[http_service]
trait UserService {
    type Error;
    #[options]
    async fn ping(&self) -> Result<Response<()>, Self::Error>;
}