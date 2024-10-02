use log::info;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Meta, NestedMeta};
use warp::Filter;

#[proc_macro_attribute]
pub fn handler(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match generate_handler(args.into(), input.into()) {
        Ok(stream) => stream.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn generate_handler(_args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let expanded = quote! {
        #[derive(Debug)]
        struct example_2;
        impl example_2 {
            pub fn new() -> Self {
                Self
            }
        }
        impl Endpoint for example_2 {
            fn handler(&self, req: &mario_core::route::request::Request) -> Result<Response, Error> {
                // Your implementation here
                //Ok(Response::new("run example handler"))
                async fn example_1() -> String {
                    //Ok(Response::new("run example_1"))
                    "run example_2".to_string()
                }
                let fut = example_1();
                let response = executor::block_on(fut);
                Ok(Response::new(&response))
            }
        }
    };
    Ok(expanded.into())
}
