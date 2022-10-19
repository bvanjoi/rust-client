use std::future::Future;

use crate::error::CoreError;
use crate::uri::uri::Uri;
use crate::uri::uri_resolver::UriResolver;
use crate::wrapper::{GetFileOptions};

pub struct UriRedirect {
  pub from: Uri,
  pub to: Uri,
}

pub struct ClientConfig {
  pub redirects: Vec<UriRedirect>,
  pub resolver: Vec<Box<dyn UriResolver>>
}

pub trait Client {
  fn get_config(&self) -> ClientConfig;
  fn get_redirects(&self) -> Vec<UriRedirect>;
  fn get_uri_resolver(&self) -> Box<dyn UriResolver>;
  fn get_file(&self, uri: &Uri, options: &GetFileOptions) -> dyn Future<Output = Result<String, CoreError>>;
}
