#![feature(custom_attribute)]
#![allow(unused_attributes)]

use interfacer_http::{
    http::Response,
    http_service,
};
trait UserService {
    type Error;
    #[options]
    fn ping<'life0, 'async_trait>(
        &'life0 self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Response<()>, Self::Error>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
impl<T: interfacer_http::HttpClient> UserService for T {
    type Error = <Self as interfacer_http::HttpClient>::Err;
    #[options]
    fn ping<'life0, 'async_trait>(
        &'life0 self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Response<()>, Self::Error>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        #[allow(clippy::used_underscore_binding)]
        async fn __ping<T: interfacer_http::HttpClient>(
            _self: &T,
        ) -> Result<Response<()>, <T as UserService>::Error> {
            #[allow(unused_imports)]
            use interfacer_http::{
                http::{header::CONTENT_TYPE, Response, StatusCode},
                mime::Mime,
                ContentInto, ToContent, Unexpected,
            };
            let _resp = _self
                .request(
                    _self
                        .helper()
                        .request()
                        .uri(
                            _self
                                .helper()
                                .parse_uri("/")?
                                .as_str(),
                        )
                        .method("OPTIONS")
                        .body(Vec::new())?,
                )
                .await?;
            if StatusCode::from_u16(200u16).unwrap() != _resp.status() {
                return Err(
                    Unexpected::new(StatusCode::from_u16(200u16).unwrap().into(), _resp).into(),
                );
            }
            Ok(Response::from_parts(_resp.into_parts().0, ()))
        }
        Box::pin(__ping::<T>(self))
    }
}

//#[http_service]
//trait UserService {
//    type Error;
//    #[options]
//    async fn ping(&self) -> Result<Response<()>, Self::Error>;
//}