use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, LitInt, Meta, MetaNameValue, NestedMeta};

pub fn progress_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let val = find_attr_efficiency(&ast, "ffxiv_progress", "efficiency").into_iter();

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl crate::actions::progress::ProgressAction for #ident {
            #(const EFFICIENCY: u16 = #val;)*
        }
    )
    .into()
}

pub fn buff_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl crate::actions::buffs::BuffAction for #ident {}
    )
    .into()
}

pub fn quality_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let val = find_attr_efficiency(&ast, "ffxiv_quality", "efficiency").into_iter();

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl crate::actions::quality::QualityAction for #ident {
            #(const EFFICIENCY: u16 = #val;)*
        }
    )
    .into()
}

pub fn cp_cost(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let val = find_attr_efficiency(&ast, "ffxiv_cp", "cost").expect(
        "Unlike some other proc macros, CP Cost requires you to specify the CP_COST value.",
    );

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl crate::actions::CpCost for #ident {
            const CP_COST: i16 = #val;
        }
    )
    .into()
}

pub fn durability(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let val = find_attr_efficiency(&ast, "ffxiv_durability", "cost").into_iter();

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl crate::actions::DurabilityFactor for #ident {
            #(const DURABILITY_USAGE: i8 = #val;)*
        }
    )
    .into()
}

pub fn can_execute(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl crate::actions::CanExecute for #ident {}
    )
    .into()
}

fn find_attr_efficiency(
    ast: &DeriveInput,
    tag_attr: &str,
    assoc_const_attr: &str,
) -> Option<LitInt> {
    for meta in ast.attrs.iter().filter_map(|v| v.parse_meta().ok()) {
        if let Meta::List(list) = meta {
            if list.path.is_ident(tag_attr) {
                for nested in list.nested {
                    match nested {
                        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Int(lit),
                            ..
                        })) if path.is_ident(assoc_const_attr) => return Some(lit),
                        _ => continue,
                    };
                }
            }
        }
    }
    None
}
