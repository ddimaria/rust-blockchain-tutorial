mod newtype;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(NewType)]
pub fn newtype(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    newtype::append(input).into()
}
