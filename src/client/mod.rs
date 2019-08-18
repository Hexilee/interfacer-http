use crate::{
    async_trait,
    http::{Request, Response},
    Error,
};
pub use helper::Helper;
pub use response::{ResponseError, ResponseExt};

// TODO: use T: AsyncRead as type of Request::Body
// TODO: use T: AsyncRead as type of Response::Body
#[async_trait]
pub trait HttpClient {
    type Err: Error;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Err>;
    fn helper(&self) -> &Helper;
}

mod helper;
mod response;
