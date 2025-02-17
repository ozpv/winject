#![warn(clippy::pedantic)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn injector(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!()
}
