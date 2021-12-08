use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DataEnum, DeriveInput, Generics, Ident,
    Lit, Meta, NestedMeta, Path, WhereClause,
};

pub(super) fn condition_derive(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let span = ast.span();

    if let Data::Enum(data) = ast.data {
        let generics = &mut ast.generics;
        generics.make_where_clause();
        let generics = &*generics;
        let where_clause = generics.where_clause.as_ref().unwrap();

        let mut out = TokenStream::new();

        let mut modifiers = vec![];
        let mut fn_names = vec![];

        let condition_types = parse_variants(&data);
        for (item, name) in condition_types {
            let modifier = format!("{}Modifier", name);
            let qualified_modifier = format!("crate::lookups::{}", modifier);
            let qualified_modifier = syn::parse(qualified_modifier.parse().unwrap()).unwrap();

            out.extend(derive_into(
                item,
                ident,
                generics,
                where_clause,
                &qualified_modifier,
            ));

            let fn_name = modifier.to_case(Case::Pascal).to_case(Case::Snake);
            let fn_name = format!("to_{}", fn_name);
            let fn_name = Ident::new(&fn_name, span);

            modifiers.push(qualified_modifier);
            fn_names.push(fn_name);
        }

        let params = &generics.params;

        let is_excellent = if data.variants.iter().any(|v| v.ident == "Excellent") {
            quote! {
                fn is_excellent(self) -> bool {
                    matches!(self, Self::Excellent)
                }
            }
        } else {
            quote! {
                fn is_excellent(self) -> bool {
                    false
                }
            }
        };

        let expert = if ast.attrs.contains(&parse_quote!(#[ffxiv(expert)])) {
            quote!(
                const EXPERT: bool = true;
            )
        } else {
            quote!(
                const EXPERT: bool = false;
            )
        };

        let main_derive: TokenStream = quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl #params crate::conditions::Condition for #ident #params #where_clause {
                #expert

                #(
                fn #fn_names (self) -> #modifiers {
                    self.into()
                }
                )*
                fn is_good(self) -> bool {
                    matches!(self, Self::Good)
                }
                #is_excellent
            }
        }
        .into();

        out.extend(main_derive);

        out
    } else {
        panic!("Can only derive Condition on enums")
    }
}

struct ConditionTypes<'a> {
    quality: Vec<&'a Ident>,
    progress: Vec<&'a Ident>,
    success: Vec<&'a Ident>,
    durability: Vec<&'a Ident>,
    status: Vec<&'a Ident>,
    cp: Vec<&'a Ident>,
}

impl<'a> IntoIterator for ConditionTypes<'a> {
    type Item = (Vec<&'a Ident>, &'static str);
    type IntoIter = std::array::IntoIter<Self::Item, 6>;

    fn into_iter(self) -> Self::IntoIter {
        [
            (self.quality, "Quality"),
            (self.progress, "Progress"),
            (self.success, "SuccessRate"),
            (self.durability, "Durability"),
            (self.status, "StatusDuration"),
            (self.cp, "CpUsage"),
        ]
        .into_iter()
    }
}

fn parse_variants(data: &DataEnum) -> ConditionTypes {
    let mut quality = vec![];
    let mut progress = vec![];
    let mut success = vec![];
    let mut durability = vec![];
    let mut status = vec![];
    let mut cp = vec![];

    for variant in &data.variants {
        for meta in variant.attrs.iter().filter_map(|v| v.parse_meta().ok()) {
            if let Meta::List(list) = meta {
                if list.path.is_ident("ffxiv") {
                    for nested in list.nested {
                        // I hate the clones but it's so much less ugly this way trust me
                        let ident_string = match nested {
                            NestedMeta::Lit(Lit::Str(lit)) => {
                                Some(lit.value().as_str().to_string())
                            }
                            NestedMeta::Meta(Meta::Path(ident)) => {
                                ident.get_ident().map(|v| v.to_string())
                            }
                            _ => {
                                panic!(
                                    "Invalid format for FFXIV Condition Derive, got {:?}",
                                    nested
                                )
                            }
                        };

                        match ident_string.as_deref() {
                            Some("quality") => quality.push(&variant.ident),
                            Some("progress") => progress.push(&variant.ident),
                            Some("success") => success.push(&variant.ident),
                            Some("durability") => durability.push(&variant.ident),
                            Some("status") => status.push(&variant.ident),
                            Some("cp") => cp.push(&variant.ident),
                            None | Some(_) => panic!(
                                "Invalid format for FFXIV Condition Derive, got {:?}",
                                ident_string
                            ),
                        }
                    }
                }
            }
        }
    }

    ConditionTypes {
        quality,
        progress,
        success,
        durability,
        status,
        cp,
    }
}

fn derive_into(
    variants: Vec<&Ident>,
    ename: &Ident,
    generics: &Generics,
    where_clause: &WhereClause,
    into: &Path,
) -> TokenStream {
    let params = &generics.params;

    let variants = variants.into_iter();
    let variants2 = variants.clone();

    quote![
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #params From<#ename #params> for #into #where_clause {
            fn from(other: #ename #params) -> #into {
                match other {
                    #(#ename::#variants => #into::#variants2,)*
                    _ => #into::Normal,
                }
            }
        }
    ]
    .into()
}
