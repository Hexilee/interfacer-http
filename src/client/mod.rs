use crate::{
    async_trait,
    http::{Request, Response},
    Error,
};
pub use helper::Helper;
pub use response::{CookieError, ResponseExt};

// TODO: use T: AsyncRead as type of Request::Body
// TODO: use T: AsyncRead as type of Response::Body
/// HttpClient trait.
/// Should be implemented by other asynchronous http client.
///
/// ### Example
///
/// ```rust,ignore
/// //! base on hyper
/// #[async_trait]
/// impl<C> HttpClient for Client<C>
/// where
///     C: Connect + Sync + 'static,
///     C::Transport: 'static,
///     C::Future: 'static,
/// {
///     type Err = Error;
///     async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>> {
///         let (parts, body) = req.into_parts();
///         let (parts, mut body) = self
///             .inner
///             .request(Request::from_parts(parts, body.into()))
///             .await?
///             .into_parts();
///         let mut data = Vec::new();
///         while let Some(chunk) = body.next().await {
///             data.extend_from_slice(&chunk?);
///         }
///         Ok(Response::from_parts(parts, data))
///     }
///
///     fn helper(&self) -> &Helper {
///         &self.helper
///     }
/// }
/// ```
/// > Refer to interfacer-http-type for full implementation.
#[async_trait]
pub trait HttpClient: Sync {
    type Err: Error;
    async fn request(&self, req: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Err>;
    fn helper(&self) -> &Helper;
}

mod helper;
mod response;
