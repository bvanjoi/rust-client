use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    client::{Client, ClientConfig, UriRedirect},
    error::Error,
    invoke::{Invoker, InvokeArgs},
    loader::Loader,
    uri::Uri,
    uri_resolution_context::UriResolutionContext,
    uri_resolver::{UriResolverHandler},
    wrapper::Wrapper,
};
use polywrap_msgpack::{decode, DeserializeOwned};
use tokio::sync::Mutex;

use crate::{wrapper_invoker::WrapperInvoker, wrapper_loader::WrapperLoader};

pub struct PolywrapClient {
    config: ClientConfig,
    loader: WrapperLoader,
    invoker: WrapperInvoker,
}

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        let loader = WrapperLoader::new(config.resolver.clone());
        let invoker = WrapperInvoker::new(loader.clone());

        Self {
            config,
            invoker,
            loader,
        }
    }

    pub async fn invoke_wrapper_and_decode<T: DeserializeOwned>(
        &self,
        wrapper: Box<dyn Wrapper>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self
            .invoke_wrapper(Arc::new(Mutex::new(wrapper)), uri, method, args, resolution_context)
            .await?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }

    pub async fn invoke_and_decode<T: DeserializeOwned>(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self.invoke(uri, method, args, resolution_context).await?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }
}

#[async_trait]
impl Invoker for PolywrapClient {
    async fn invoke(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        self.invoker.invoke(uri, method, args, resolution_context).await
    }

    async fn invoke_wrapper(
        &self,
        wrapper: Arc<Mutex<Box<dyn Wrapper>>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        self.invoker.invoke_wrapper(wrapper, uri, method, args, resolution_context).await
    }
}

#[async_trait(?Send)]
impl Client for PolywrapClient {
    fn get_config(&self) -> &ClientConfig {
        &self.config
    }

    fn get_redirects(&self) -> &Vec<UriRedirect> {
        &self.config.redirects
    }

    // async fn get_file(&self, uri: &Uri, options: &GetFileOptions) -> Result<Vec<u8>, Error> {
    //     let load = self.load_wrapper(uri, Option::None).await;

    //     match load {
    //         Ok(wrapper) => {
    //             let result = wrapper.get_file(options);
    //             return result;
    //         }
    //         Err(err) => {
    //             return Err(Error::GetFileError(format!(
    //                 "Failed to load wrapper: {}",
    //                 err
    //             )));
    //         }
    //     }
    // }
}

#[async_trait]
impl UriResolverHandler for PolywrapClient {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<polywrap_core::uri_resolution_context::UriPackageOrWrapper, Error> {
        self.loader.try_resolve_uri(uri, resolution_context).await
    }
}

#[async_trait]
impl Loader for PolywrapClient {
    async fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Box<dyn Wrapper>, Error> {
        self.loader.load_wrapper(uri, resolution_context).await
    }
}
