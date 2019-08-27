#![feature(custom_attribute, param_attrs)]
#![allow(unused_attributes)]

use interfacer_http::{
    http::{header::COOKIE, Request, Response},
    http_service, mime,
    url::Url,
};
use mock::{Client, Error};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: i32,
}

const MOCK_BASE_URL: &str = "https://mock.rs";

#[rustfmt::skip]
#[http_service]
trait UserService {
    type Error;

    #[options]
    async fn ping(&self) -> Result<Response<()>, Self::Error>;

    #[get("/api/user/{id}")]
    async fn get_user(&self, id: u64) -> Result<Response<User>, Self::Error>;

    #[get("/api/user?age_max={age_max}")]
    async fn get_users(&self, age_max: u8) -> Result<Response<Vec<User>>, Self::Error>;

    #[put("/api/user/{id}")]
    async fn put_user(
        &self,
        id: u64,
        #[body] user: &User,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<User>, Self::Error>;

    #[post("/api/user", mime::APPLICATION_WWW_FORM_URLENCODED)]
    #[expect(201, mime::APPLICATION_MSGPACK)]
    async fn post_user(
        &self,
        #[body] user: &User,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<User>, Self::Error>;

    #[post("/api/users")]
    #[expect(201)]
    async fn post_users(
        &self,
        #[body] users: Vec<User>,
        #[header(COOKIE)] cookie: &str,
    ) -> Result<Response<()>, Self::Error>;
}

async fn ping_handler(req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Error> {
    assert_eq!(Url::parse(MOCK_BASE_URL)?.join("/")?.as_str(), req.uri());
    assert_eq!("OPTIONS", req.method());
    Ok(Response::builder().status(200).body(Vec::new())?)
}

#[tokio::test]
async fn test_ping() -> Result<(), Error> {
    let service = Client::new(MOCK_BASE_URL.parse()?, ping_handler);
    let resp = service.ping().await?;
    assert_eq!(200, resp.status());
    Ok(())
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
}
