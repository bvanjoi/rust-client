use std::{collections::HashMap, sync::Arc};

use polywrap_manifest::versions::WrapManifest;
use serde_json::Value;

use crate::{error::Error, uri::Uri};

pub trait PluginModule {
    fn get_manifest(&self) -> Result<WrapManifest, Error>;
    fn get_methods_map(
        &self,
    ) -> &HashMap<
        String,
        fn(
            Arc<dyn PluginModule>,
            Value,
            Arc<dyn crate::invoke::Invoker>,
        ) -> Result<Value, Error>,
    >;
    fn get_method(
        self: Arc<Self>,
        method_name: &str,
    ) -> Result<
        fn(
            Arc<dyn PluginModule>,
            Value,
            Arc<dyn crate::invoke::Invoker>,
        ) -> Result<Value, Error>,
        Error,
    > {
        if let Some(func) = self.get_methods_map().get(method_name).map(|x| *x) {
            Ok(func)
        } else {
            Err(Error::MissingPluginMethodError(method_name.to_string()))
        }
    }
    fn _wrap_invoke(
        self: Arc<Self>,
        method_name: &str,
        params: Value,
        invoker: Arc<dyn crate::invoke::Invoker>,
    ) -> Result<Value, Error>
    where
        Self: Sized + 'static,
    {
        let method = self.clone().get_method(method_name)?;
        Ok(method(self, params, invoker)?)
    }
}

pub struct PluginRegistration {
    pub uri: Uri,
    pub plugins: Vec<Box<dyn PluginModule>>,
}
