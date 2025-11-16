extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod derive;
mod internals;

#[proc_macro_derive(Gatekeeper, attributes(keep))]
pub fn derive_gatekeeper(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    derive::expand_derive_gatekeeper(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
