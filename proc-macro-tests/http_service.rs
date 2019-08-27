#![feature(custom_attribute, param_attrs)]
#![allow(unused_attributes)]

use interfacer_http::{
    http::{header::CONTENT_TYPE, header::COOKIE, Request, Response},
    http_service, mime,
    mock::{Client, Error},
    url::Url,
    ToContent,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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
    #[expect(200, mime::APPLICATION_JSON)]
    async fn get_user(&self, id: u64) -> Result<Response<User>, Self::Error>;

    #[get("/api/user?age_max={age_max}")]
    #[expect(200, mime::APPLICATION_JSON)]
    async fn get_users(&self, age_max: u8) -> Result<Response<Vec<User>>, Self::Error>;

    #[put("/api/user/{id}", mime::APPLICATION_JSON)]
    #[expect(200, mime::APPLICATION_JSON)]
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

    #[post("/api/users", mime::APPLICATION_JSON)]
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

async fn get_user_handler(req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Error> {
    assert_eq!(
        Url::parse(MOCK_BASE_URL)?.join("/api/user/0")?.as_str(),
        req.uri()
    );
    assert_eq!("GET", req.method());
    Ok(Response::builder()
        .status(200)
        .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(
            User {
                name: "hexi".to_string(),
                age: 20,
            }
            .to_content(&mime::APPLICATION_JSON)?,
        )?)
}

#[tokio::test]
async fn test_get_user() -> Result<(), Error> {
    let service = Client::new(MOCK_BASE_URL.parse()?, get_user_handler);
    let resp = service.get_user(0).await?;
    assert_eq!(200, resp.status());
    assert_eq!(
        &User {
            name: "hexi".to_string(),
            age: 20,
        },
        resp.body()
    );
    Ok(())
}

async fn get_users_handler(req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Error> {
    assert_eq!(
        Url::parse(MOCK_BASE_URL)?
            .join("/api/user?age_max=40")?
            .as_str(),
        req.uri()
    );
    assert_eq!("GET", req.method());
    Ok(Response::builder()
        .status(200)
        .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(
            vec![
                User {
                    name: "hexi".to_string(),
                    age: 20,
                },
                User {
                    name: "boss".to_string(),
                    age: 35,
                },
            ]
            .to_content(&mime::APPLICATION_JSON)?,
        )?)
}

#[tokio::test]
async fn test_get_users() -> Result<(), Error> {
    let service = Client::new(MOCK_BASE_URL.parse()?, get_users_handler);
    let resp = service.get_users(40).await?;
    assert_eq!(200, resp.status());
    assert_eq!(
        &vec![
            User {
                name: "hexi".to_string(),
                age: 20,
            },
            User {
                name: "boss".to_string(),
                age: 35,
            },
        ],
        resp.body()
    );
    Ok(())
}
