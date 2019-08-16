use crate::{
    async_trait,
    http::{Request, Response},
    url::Url,
    RequestFail,
};

use std::future::Future;

// TODO: use T: AsyncRead as type of Request::Body
// TODO: use T: AsyncRead as type of Response::Body
#[async_trait]
pub trait HttpClient {
    type Err: Into<RequestFail>;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Err>;
}

pub trait HttpService {
    type Client: HttpClient;
    fn get_base_url(&self) -> &Url;
    fn get_client(&self) -> &Self::Client;
}

#[async_trait]
impl<T, F, E> HttpClient for T
where
    E: Into<RequestFail> + 'static,
    F: Future<Output=Result<Response<Vec<u8>>, E>> + Send + 'static,
    T: Fn(Request<Vec<u8>>) -> F + Sync,
{
    type Err = E;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Err> {
        self(req).await
    }
}

pub mod response;
