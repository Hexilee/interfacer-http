use crate::{async_trait, http::Request, RequestFail};

pub use config::Config;
pub use response::Response;

// TODO: use T: AsyncRead as type of Request::Body
// TODO: use T: AsyncRead as type of Response::Body
#[async_trait]
pub trait HttpClient {
    type Err: Into<RequestFail>;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Err>;
    fn config(&self) -> &Config;
}

mod config;
mod response;
