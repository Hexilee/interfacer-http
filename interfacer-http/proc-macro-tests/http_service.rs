#![cfg(feature = "derive")]
#![feature(custom_attribute, async_await, param_attrs)]
#![allow(unused_attributes)]

use interfacer_http::derive::{FromContent, ToContent};
use interfacer_http::{content_types, http_service, Response, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromContent, ToContent, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}

#[http_service]
trait UserService: Clone {
    #[put("/api/user/{uid}?age={age}")]
    #[expect(200, content_types::APPLICATION_JSON)]
    async fn put_user(&self, id: u64, age: i32, user: &User) -> Result<Response<User>> {}
}

//#[interfacer_http::async_trait]
//trait UserService: Clone + interfacer_http::HttpService + core::marker::Sync {
//    #[put("/api/user/{uid}?age={age}")]
//    #[expect(200, content_types::APPLICATION_JSON)]
//    async fn put_user(&self, id: u64, age: i32,
//                      user: &User) -> Result<Response<User>> {
//        use interfacer_http::{RequestFail, ContentType,
//                              http::{StatusCode, header::CONTENT_TYPE,
//                                     Response}, IntoStruct, ToContent,
//                              HttpClient, StringError};
//        use std::convert::TryInto;
//        let final_uri_ident =
//            self.get_base_url().join(&format!("/api/user/{}?age={}", &id, &age))?;
//        let expect_content_type_ident: ContentType =
//            content_types::APPLICATION_JSON.try_into()?;
//        let mut builder = interfacer_http::http::Request::builder();
//        let request_ident =
//            builder.uri(final_uri_ident.as_str()).method("put").body(Vec::new())?;
//        let (parts_ident, body_ident) =
//            self.get_client().request(request_ident).await.map_err(|err|
//                err.into())?.into_parts();
//        RequestFail::expect_status(StatusCode::from_u16(200u16).unwrap(),
//                                   parts_ident.status)?;
//        let ret_content_type =
//            parts_ident.headers.get(CONTENT_TYPE).ok_or(StringError::new("cannot get Content-Type from response headers"))?;
//        expect_content_type_ident.expect(&ContentType::from_header(ret_content_type)?)?;
//        Ok(Response::from_parts(parts_ident,
//                                body_ident.into_struct(&expect_content_type_ident)?).into())
//    }
//}
//
//#[interfacer_http::async_trait]
//impl<T: Clone + interfacer_http::HttpService + core::marker::Sync>
//UserService for T {}
//
