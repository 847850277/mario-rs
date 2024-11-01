use proc_macro::TokenStream;

use quote::quote;
use syn::ItemFn;

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
    let item_fn = syn::parse::<ItemFn>(input)?;
    let vis = &item_fn.vis;
    let ident = &item_fn.sig.ident;
    let expanded = quote! {

        #[derive(Debug)]
        #vis struct #ident;

        impl #ident {
            pub fn new() -> Self {
                Self
            }
        }

        impl Endpoint for #ident {
            fn call(&self, req: &mario_core::route::request::Request) -> impl Future<Output=Result<Response<String>, Error>> + Send {
                // Your implementation here
                //Ok(Response::new("run example handler"))
                // async fn example_2() -> String {
                //     //Ok(Response::new("run example_1"))
                //     "run example_2".to_string()
                // }
                #item_fn
                let fut = #ident();
                async move {
                    let response = fut.await;
                    Ok(Response::new(response.to_string()))
                    // let response = example_1().await;
                    // Ok(Response::new(response.to_string()))
                }

            }
        }
    };
    Ok(expanded.into())
}
