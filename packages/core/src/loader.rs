use async_trait::async_trait;

use crate::{wrapper::Wrapper, error::Error, uri_resolution_context::UriResolutionContext, uri::Uri, uri_resolver::UriResolverHandler};

#[async_trait]
pub trait Loader: UriResolverHandler + Send + Sync {
    async fn load_wrapper(&self, uri: &Uri, resolution_context: Option<&mut UriResolutionContext>,) -> Result<Box<dyn Wrapper>, Error>;
}