use crate::internals::Ctxt;
use proc_macro2::TokenStream;

pub fn expand_derive_gatekeeper(_input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();

    ctxt.check()?;

    todo!()
}
