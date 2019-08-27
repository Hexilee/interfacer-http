#![feature(custom_attribute, param_attrs)]
#![allow(unused_attributes)]

use interfacer_http::{
    http::{header::COOKIE, Response},
    http_service, mime,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: i32,
}

#[rustfmt::skip]
#[http_service]
trait UserService {
    type Error;
    #[put("/api/user/{id}?age={age}")]
    #[expect(200, mime::APPLICATION_JSON)]
    async fn put_user(
        &self,
        id: u64,
        age: i32,
        #[body] user: &User,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<User>, Self::Error>;
}

mod mock {
    use interfacer_http::{
        async_trait,
        http::{self, header::CONTENT_TYPE, Request, Response, Version},
        url::{self, Url},
        FromContentError, Helper, HttpClient, ToContentError, Unexpected,
    };
    use std::future::Future;

    use derive_more::{Display, From};

    #[derive(Display, Debug, From)]
    pub enum Error {
        #[display(fmt = "url parse error: {}", _0)]
        UrlParseError(url::ParseError),

        #[display(fmt = "http error: {}", _0)]
        HttpError(http::Error),

        #[display(fmt = "to content error: {}", _0)]
        ToContentError(ToContentError),

        #[display(fmt = "from content error: {}", _0)]
        FromContentError(FromContentError),

        #[display(fmt = "{}", _0)]
        Unexpected(Unexpected),
    }

    impl std::error::Error for Error {}

    pub type Result<T> = std::result::Result<T, Error>;

    pub struct Client<F> {
        helper: Helper,
        handler: fn(Request<Vec<u8>>) -> F,
    }

    impl<F> Client<F>
    where
        F: Future<Output = Result<Response<Vec<u8>>>> + Send + 'static,
    {
        pub fn new(base_url: Url, handler: fn(Request<Vec<u8>>) -> F) -> Self {
            Self {
                handler,
                helper: Helper::new().with_base_url(base_url),
            }
        }
    }

    #[async_trait]
    impl<F> HttpClient for Client<F>
    where
        F: Future<Output = Result<Response<Vec<u8>>>> + Send + 'static,
    {
        type Err = Error;
        async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
            (self.handler)(req).await
        }

        fn helper(&self) -> &Helper {
            &self.helper
        }
    }

    //    async fn login_page_handler(req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
    //        assert_eq!(
    //            Url::parse(BASE_URL)?.join("default2.aspx")?.as_str(),
    //            req.uri()
    //        );
    //        assert_eq!("GET", req.method());
    //        Ok(Response::builder()
    //            .status(200)
    //            .version(Version::HTTP_11)
    //            .header(CONTENT_TYPE, "text/html; charset=gb2312")
    //            .body(
    //                encoding_from_whatwg_label("gb2312")
    //                    .unwrap()
    //                    .encode(LOGIN_PAGE, EncoderTrap::Strict)
    //                    .unwrap(),
    //            )?)
    //    }
    //
    //    #[tokio::test]
    //    async fn test_login_page() -> Result<()> {
    //        let service = Client::new(BASE_URL.parse()?, login_page_handler);
    //        let page = service.get_login_page().await?;
    //        assert_eq!(
    //            &page.body().hidden_form,
    //            &HiddenForm {
    //                event_argument: "".into(),
    //                event_target: "".into(),
    //                view_state: "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=".into(),
    //            }
    //        );
    //        Ok(())
    //    }
}
