use crate::{async_trait, Request, RequestFail, Response, StdResult, Url};

// TODO: use T: AsyncRead as type of Request::Body
// TODO: use T: AsyncRead as type of Response::Body
#[async_trait]
pub trait HttpClient {
    type Err: Into<RequestFail>;
    async fn request(&self, req: Request<Vec<u8>>) -> StdResult<Response<Vec<u8>>, Self::Err>;
}

pub trait HttpService {
    type Client: HttpClient;
    fn get_base_url(&self) -> &Url;
    fn get_client(&self) -> &Self::Client;
}
