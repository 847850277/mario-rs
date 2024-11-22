use proc_macro::TokenStream;

use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn handler(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate_handler(args, input).unwrap_or_else(|err| err.into_compile_error().into())
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
            fn call(&self, req: &mario_core::request::Request) -> Result<Response<String>, Error>{
                #item_fn
                let fut = #ident();
                let response = fut;
                Ok(Response::new(response.to_string()))


            }
        }
    };
    Ok(expanded.into())
}
