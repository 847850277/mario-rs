use log::info;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // This is a placeholder. You need to replace this with the actual implementation of your route macro.
    info!("{:?}", item);
    item
}
