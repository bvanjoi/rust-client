use polywrap_client::{
    builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler},
    core::{package::WrapPackage, resolvers::uri_resolver_like::UriResolverLike, uri::Uri}, client::PolywrapClient,
};
use std::sync::{Arc, Mutex};

use crate::{
    plugin_wrapper::FFIPluginWrapper,
    resolvers::{
        _static::FFIStaticUriResolver,
        extendable::FFIExtendableUriResolver,
        ffi_resolver::{FFIUriResolver, FFIUriResolverWrapper},
        recursive::FFIRecursiveUriResolver,
    },
    wasm_wrapper::FFIWasmWrapper, client::FFIClient,
};

pub struct FFIBuilderConfig {
    pub inner_builder: Mutex<BuilderConfig>,
}

impl FFIBuilderConfig {
    pub fn new() -> FFIBuilderConfig {
        FFIBuilderConfig {
            inner_builder: Mutex::new(BuilderConfig::new(None)),
        }
    }

    pub fn add_env(&self, uri: Arc<Uri>, env: &str) {
        self.inner_builder.lock().unwrap().add_env(
            uri.as_ref().clone(),
            serde_json::from_str(env).unwrap(),
        );
    }

    pub fn remove_env(&self, uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_env(uri.as_ref().clone());
    }

    pub fn set_env(&self, uri: Arc<Uri>, env: &str) {
        self.inner_builder.lock().unwrap().set_env(
            uri.as_ref().clone(),
            serde_json::from_str(env).unwrap(),
        );
    }

    pub fn add_interface_implementation(&self, interface_uri: Arc<Uri>, implementation_uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_interface_implementation(
                interface_uri.as_ref().clone(),
                implementation_uri.as_ref().clone(),
            );
    }

    pub fn remove_interface_implementation(&self, interface_uri: Arc<Uri>, implementation_uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_interface_implementation(
                interface_uri.as_ref().clone(),
                implementation_uri.as_ref().clone()
            );
    }

    pub fn add_wasm_wrapper(&self, uri: Arc<Uri>, wrapper: Arc<FFIWasmWrapper>) {
        self.inner_builder.lock().unwrap().add_wrapper(
            uri.as_ref().clone(),
            wrapper.inner_wasm_wrapper.clone(),
        );
    }

    pub fn add_plugin_wrapper(&self, uri: Arc<Uri>, wrapper: Arc<FFIPluginWrapper>) {
        self.inner_builder.lock().unwrap().add_wrapper(
            uri.as_ref().clone(),
            wrapper.inner_plugin.clone(),
        );
    }

    pub fn remove_wrapper(&self, uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_wrapper(uri.as_ref().clone());
    }

    pub fn add_package(&self, uri: Arc<Uri>, package: Box<dyn WrapPackage>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_package(uri.as_ref().clone(), Arc::new(Mutex::new(package)));
    }

    pub fn remove_package(&self, uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_package(uri.as_ref().clone());
    }

    pub fn add_redirect(&self, from: Arc<Uri>, to: Arc<Uri>) {
        self.inner_builder.lock().unwrap().add_redirect(
            from.as_ref().clone(),
            to.as_ref().clone(),
        );
    }

    pub fn remove_redirect(&self, from: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_redirect(from.as_ref().clone());
    }

    pub fn add_resolver(&self, resolver: Box<dyn FFIUriResolver>) {
        let resolver: FFIUriResolverWrapper = resolver.into();
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn add_static_resolver(&self, resolver: FFIStaticUriResolver) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn add_extendable_resolver(&self, resolver: FFIExtendableUriResolver) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn add_recursive_resolver(&self, resolver: FFIRecursiveUriResolver) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn build(&self) -> FFIClient {
      let config = self.inner_builder.lock().unwrap().clone().build();
      let client = PolywrapClient::new(config);
      FFIClient::new(client)
    }
}
