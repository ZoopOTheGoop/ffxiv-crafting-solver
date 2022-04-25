use darling::{util::Flag, FromDeriveInput, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromMeta)]
struct ConditionCollectionMeta {
    #[darling(default)]
    expert: Flag,
    bits: syn::Path,
}

#[derive(FromMeta, Debug, Copy, Clone)]
#[darling(rename_all = "snake_case")]
enum ConditionMeta {
    Normal,
    Quality,
    Cp,
    Durability,
    Success,
    Progress,
    Status,
}

impl Default for ConditionMeta {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(FromVariant, Debug, Clone)]
#[darling(attributes(ffxiv))]
struct ConditionVariant {
    ident: syn::Ident,
    #[darling(default)]
    affects: ConditionMeta,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(ffxiv))]
struct ConditionDerive {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<ConditionVariant, ()>,
    condition: ConditionCollectionMeta,
}

macro_rules! gen_fun {
    ($it:expr, $condition:path, $fn_name:ident, $modifier:path, $collection:ident) => {
        let filtered = filter_conds!($it, $condition);
        let tt = quote!(
            fn $fn_name(self) -> crate::lookups::$modifier {
                match self {
                    #(
                    Self::#filtered => crate::lookups::$modifier::#filtered,
                    )*
                    _ => crate::lookups::$modifier::Normal,
                }
            }
        );

        $collection.extend(tt);
    };
}

macro_rules! filter_conds {
    ($it:expr, $condition:path) => {
        $it.filter_map(|variant| {
            if matches!(variant.affects, $condition) {
                Some(&variant.ident)
            } else {
                None
            }
        })
    };
}

pub(super) fn condition_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let condition_enum: ConditionDerive = match FromDeriveInput::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let variants = match condition_enum.data.take_enum() {
        Some(v) => v,
        None => {
            return darling::Error::custom("Condition derive only works on enums")
                .with_span(&condition_enum.ident.span())
                .write_errors()
                .into()
        }
    };

    let mut functions = quote!();

    gen_fun!(
        variants.iter(),
        ConditionMeta::Quality,
        to_quality_modifier,
        QualityModifier,
        functions
    );

    gen_fun!(
        variants.iter(),
        ConditionMeta::Cp,
        to_cp_usage_modifier,
        CpUsageModifier,
        functions
    );

    gen_fun!(
        variants.iter(),
        ConditionMeta::Durability,
        to_durability_modifier,
        DurabilityModifier,
        functions
    );

    gen_fun!(
        variants.iter(),
        ConditionMeta::Progress,
        to_progress_modifier,
        ProgressModifier,
        functions
    );

    gen_fun!(
        variants.iter(),
        ConditionMeta::Status,
        to_status_duration_modifier,
        StatusDurationModifier,
        functions
    );

    gen_fun!(
        variants.iter(),
        ConditionMeta::Success,
        to_success_rate_modifier,
        SuccessRateModifier,
        functions
    );

    functions.extend(quote!(
        fn is_good(self) -> bool {
            matches!(self, Self::Good)
        }
    ));

    if variants.iter().any(|variant| variant.ident == "Excellent") {
        functions.extend(quote!(
            fn is_excellent(self) -> bool {
                matches!(self, Self::Excellent)
            }
        ));
    }

    let (impl_generic, type_generic, where_clause) = condition_enum.generics.split_for_impl();

    let name = condition_enum.ident;
    let expert = bool::from(condition_enum.condition.expert);
    let bits = condition_enum.condition.bits;

    quote!(
        #[automatically_derived]
        impl #impl_generic Condition for #name #type_generic #where_clause {
            const EXPERT: bool = #expert;
            const BITS: ConditionBits = crate::conditions::ConditionBits(crate::lookups::#bits);

            #functions
        }
    )
    .into()
}
