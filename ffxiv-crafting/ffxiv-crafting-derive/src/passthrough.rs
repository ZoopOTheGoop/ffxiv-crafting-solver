use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, parse_quote, ItemEnum};

pub fn magic_action_passthrough(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as ItemEnum);

    let attrs = target.attrs.into_iter();
    let (impl_generic, _type_generic, where_clause) = target.generics.split_for_impl();
    let where_clause = where_clause.into_iter();

    let variants = target.variants.iter();
    let variants_mirror = variants.clone().cloned().map(|mut v| {
        v.attrs = vec![
            parse_quote!(#[doc="The concrete action this defers to. Click it to see the docs for this action."]),
        ];
        v
    });

    let ident = target.ident;
    let vis = target.vis;

    quote!(
        #(#attrs)*
        #vis enum #ident #impl_generic #(#where_clause)* {
            #(#variants(#variants_mirror),)*
        }
    )
    .into()
}
