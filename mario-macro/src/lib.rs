use log::info;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Meta, NestedMeta};
use warp::Filter;

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    // This is a placeholder. You need to replace this with the actual implementation of your route macro.
    // info!("{:?}", item);
    //
    // item
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemFn);

    // Parse the attribute arguments
    let args = parse_macro_input!(args as AttributeArgs);

    // Extract the route and method from the attribute arguments
    let mut route = String::new();
    let mut method = String::new();
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("path") {
                if let syn::Lit::Str(lit) = nv.lit {
                    route = lit.value();
                }
            } else if nv.path.is_ident("method") {
                if let syn::Lit::Str(lit) = nv.lit {
                    method = lit.value();
                }
            }
        }
    }

    // Get the function name
    let fn_name = &input.sig.ident;

    // Generate the new function code
    // let expanded = quote! {
    //     #[get(#route)]
    //     async fn #fn_name() -> impl Responder {
    //         format!("You hit: {}", #route)
    //     }
    // };

    // let expanded = quote! {
    //     pub async fn #fn_name() -> impl warp::Reply {
    //         warp::path!(#route)
    //             .map(|| format!("You hit: {}", #route))
    //     }
    // };

    // Generate the new function code
    let expanded = quote! {
        pub async fn #fn_name() -> String {
            format!("You hit: {}", #route)
        }
    };

    let route = quote! {
        let route = warp::path!(#route).and_then(#fn_name);
    };

    // Return the generated code
    TokenStream::from(expanded)
}
