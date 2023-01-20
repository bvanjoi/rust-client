use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_client_builder::types::{BuilderConfig, ClientConfigHandler};
use polywrap_resolvers::{uri_resolver_wrapper::UriResolverWrapper};

use polywrap_core::{uri::Uri, resolvers::{uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}, resolver_with_history::ResolverWithHistory}};
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_wrapper::WasmWrapper;

mod helpers;

#[tokio::test]
async fn test_uri_resolver_wrapper() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let wrapper_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);
    let wrapper_uri = Uri::try_from(format!("fs/{}", wrapper_path)).unwrap();
    let builder = BuilderConfig::new(None);
    let config = builder.build();
    let client = PolywrapClient::new(config);
    
    let mut uri_resolution_context = UriResolutionContext::new();
    let uri_resolver_wrapper = UriResolverWrapper::new(
        Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap()
    );
    let result = uri_resolver_wrapper._try_resolve_uri(
        &wrapper_uri.clone(), 
        &client.loader, 
        &mut uri_resolution_context
    ).await.unwrap();
    
    if let UriPackageOrWrapper::Wrapper(_, w) = result {
        let wasm_wrapper = (&w as &dyn std::any::Any).downcast_ref::<WasmWrapper>();
        if let Some(wrapper) = wasm_wrapper {
            let mock_wrapper = helpers::get_mock_wrapper();
            assert_eq!(wrapper, &mock_wrapper);
        };
    }
    
}

#[tokio::test]
async fn recursive_uri_resolver() {
    let wrapper_github_path = "https://raw.githubusercontent.com/polywrap/wasm-test-harness/v0.2.1/wrappers/subinvoke/00-subinvoke/implementations/as";
    let http_wrapper_uri = Uri::try_from(format!("http/{}", wrapper_github_path)).unwrap();
    
    let builder = BuilderConfig::new(None);
    let config = builder.build();
    let client = PolywrapClient::new(config);
    
    let mut uri_resolution_context = UriResolutionContext::new();
    let result = client.loader.resolver.try_resolve_uri(
        &http_wrapper_uri.clone(), 
        &client.loader, 
        &mut uri_resolution_context
    ).await;

    if let Ok(r) = result {
        if let UriPackageOrWrapper::Wrapper(_, w) = r {
            let wasm_wrapper = (&w as &dyn std::any::Any).downcast_ref::<WasmWrapper>();
            if let Some(wrapper) = wasm_wrapper {
                let mock_wrapper = helpers::get_mock_wrapper();
                assert_eq!(wrapper, &mock_wrapper);
            };
        }
    }
}