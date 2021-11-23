use std::collections::{HashMap, HashSet};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, LitInt, LitStr, Meta, MetaNameValue, NestedMeta};

const EFFICIENCY: &str = "efficiency";
const COST: &str = "cost";
const BONUS: &str = "bonus";
const LEVEL: &str = "level";
const CHANCE: &str = "fail_rate";
const CLASS: &str = "class";

pub fn progress_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_progress";

    let efficiency = [(
        EFFICIENCY,
        Box::new(attr_literal(EFFICIENCY)) as Box<FfxivAttrMatcher>,
    )]
    .into_iter()
    .collect();

    let val = find_attributes(&ast, TAG, efficiency);

    let val = val.get(EFFICIENCY).into_iter().map(|v| v.to_lit_int());

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generic crate::actions::progress::ProgressAction for #ident #type_generic #(#where_clause)* {
            #(const EFFICIENCY: u16 = #val;)*
        }
    )
    .into()
}

pub fn buff_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_buff_act";
    const MAGNITUDE: &str = "amount";

    let class = [
        (
            CLASS,
            Box::new(attr_literal(CLASS)) as Box<FfxivAttrMatcher>,
        ),
        (MAGNITUDE, Box::new(attr_literal(MAGNITUDE))),
    ]
    .into_iter()
    .collect();

    let val = find_attributes(&ast, TAG, class);

    let magnitude: u8 = val
        .get(MAGNITUDE)
        .map(|v| {
            v.to_lit_int()
                .base10_parse()
                .expect("Literal should be integer")
        })
        .unwrap_or(1);

    let buff_impl = val
        .get(CLASS)
        .into_iter()
        .map(|v| v.to_lit_str())
        .filter(|v| &*v.value() == "touch")
        .map(|_| {
            quote!(
                fn buff<C, M>(&self, state: &crate::CraftingState<C, M>, so_far: &mut crate::BuffState)
                where
                    C: Condition,
                    M: QualityMap,
                {
                    so_far.quality.inner_quiet += #magnitude;
                }
            )
        });

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generic crate::actions::buffs::BuffAction for #ident #type_generic #(#where_clause)* {
            #(#buff_impl)*
        }
    )
    .into()
}

pub fn quality_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_quality";

    let efficiency = [(
        EFFICIENCY,
        Box::new(attr_literal(EFFICIENCY)) as Box<FfxivAttrMatcher>,
    )]
    .into_iter()
    .collect();

    let val = find_attributes(&ast, TAG, efficiency);

    let val = val.get(EFFICIENCY).into_iter().map(|v| v.to_lit_int());

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generic crate::actions::quality::QualityAction for #ident #type_generic #(#where_clause)* {
            #(const EFFICIENCY: u16 = #val;)*
        }
    )
    .into()
}

pub fn cp_cost(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_cp";

    let cost = [
        (COST, Box::new(attr_literal(COST)) as Box<FfxivAttrMatcher>),
        (
            BONUS,
            Box::new(attr_literal(BONUS)) as Box<FfxivAttrMatcher>,
        ),
    ]
    .into_iter()
    .collect();

    let vals = find_attributes(&ast, TAG, cost);

    let cost = vals
        .get(COST)
        .map(|v| -v.to_lit_int().base10_parse::<i16>().unwrap());
    let bonus = vals
        .get(BONUS)
        .map(|v| v.to_lit_int().base10_parse().unwrap());

    if cost.is_some() && bonus.is_some() {
        panic!("Specify either cost or bonus");
    }

    if cost.is_some() && cost > Some(0) {
        panic!(
            "Use \"bonus\" instead of cost for positive values. \
        \"cost\" is how much the move uses, \"bonus\" how much it gives you."
        )
    }

    if bonus.is_some() && bonus < Some(0) {
        panic!(
            "Use \"cost\" instead of bonus for negative values. \
        \"cost\" is how much the move uses, \"bonus\" how much it gives you."
        )
    }

    let val = cost.or(bonus).expect("Unlike some other proc macros, CpCost requires you to specify the LEVEL value as `cost` or `bonus`.");

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #type_generic crate::actions::CpCost for #ident #impl_generic #(#where_clause)* {
            const CP_COST: i16 = -#val;
        }
    )
    .into()
}

pub fn durability(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_durability";

    let cost = [
        (COST, Box::new(attr_literal(COST)) as Box<FfxivAttrMatcher>),
        (
            BONUS,
            Box::new(attr_literal(BONUS)) as Box<FfxivAttrMatcher>,
        ),
    ]
    .into_iter()
    .collect();

    let vals = find_attributes(&ast, TAG, cost);

    let cost = vals
        .get(COST)
        .map(|v| -v.to_lit_int().base10_parse::<i8>().unwrap());
    let bonus = vals
        .get(BONUS)
        .map(|v| v.to_lit_int().base10_parse().unwrap());

    if cost.is_some() && bonus.is_some() {
        panic!("Specify either cost or bonus");
    }

    if cost.is_some() && cost > Some(0) {
        panic!(
            "Use \"bonus\" instead of cost for positive values. \
        \"cost\" is how much the move uses, \"bonus\" how much it gives you."
        )
    }

    if bonus.is_some() && bonus < Some(0) {
        panic!(
            "Use \"cost\" instead of bonus for negative values. \
        \"cost\" is how much the move uses, \"bonus\" how much it gives you."
        )
    }

    let val = cost.or(bonus).into_iter();

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generic crate::actions::DurabilityFactor for #ident #type_generic #(#where_clause)* {
            #(const DURABILITY_USAGE: i8 = -#val;)*
        }
    )
    .into()
}

pub fn can_execute(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_can_exe";

    let class = [(
        CLASS,
        Box::new(attr_literal(CLASS)) as Box<FfxivAttrMatcher>,
    )]
    .into_iter()
    .collect();

    let val = find_attributes(&ast, TAG, class);

    let can_execute_impl = val.get(CLASS).into_iter().map(|v| v.to_lit_str()).map(|v| {
        let condition = match &*v.value() {
            "good_excellent" => quote!(state.condition.is_good() || state.condition.is_excellent()),
            "first_step" => quote!(state.first_step),
            _ => panic!("Unsupported condition for \"can execute\"."),
        };
        quote!(
            fn can_execute<C, M>(&self, state: &crate::CraftingState<C, M>) -> bool
            where
                C: Condition,
                M: QualityMap,
            {
                #condition
            }
        )
    });

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generic crate::actions::CanExecute for #ident #type_generic #(#where_clause)* {
            #(#can_execute_impl)*
        }
    )
    .into()
}

pub fn action_level(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_act_lvl";

    let level = [(
        LEVEL,
        Box::new(attr_literal(LEVEL)) as Box<FfxivAttrMatcher>,
    )]
    .into_iter()
    .collect();

    let val = find_attributes(&ast, TAG, level);

    let val = val.get(LEVEL).map(|v| v.to_lit_int())
    .expect( "Unlike some other proc macros, ActionLevel requires you to specify the LEVEL value as `level`.");

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generic crate::actions::ActionLevel for #ident #type_generic #(#where_clause)* {
            const LEVEL: u16 = #val;
        }
    )
    .into()
}

pub fn random_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = &ast.generics.split_for_impl();
    let where_clause = where_clause.iter();

    const TAG: &str = "ffxiv_rand_act";

    let attrs = [
        (
            CHANCE,
            Box::new(attr_literal(CHANCE)) as Box<FfxivAttrMatcher>,
        ),
        (
            CLASS,
            Box::new(attr_literal(CLASS)) as Box<FfxivAttrMatcher>,
        ),
    ]
    .into_iter()
    .collect();

    let attrs = find_attributes(&ast, TAG, attrs);
    let chance = attrs.get(CHANCE).into_iter().map(|v| v.to_lit_int());

    let fail_rate_class = attrs
        .get(CLASS)
        .into_iter()
        .map(|v| v.to_lit_str())
        .filter(|v| &*v.value() == "combo_observe")
        .map(|_| {
            quote!(
                fn fail_rate<C: Condition, M: QualityMap>(
                    &self,
                    state: &CraftingState<C, M>,
                ) -> u8 {
                    if !state.last_state_was_observation {
                        Self::FAIL_RATE
                    } else {
                        0
                    }
                }
            )
        });

    quote!(
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generic crate::actions::RandomAction for #ident #type_generic #(#where_clause)* {
            #(const FAIL_RATE: u8 = #chance;)*
            type FailAction = crate::actions::failure::NullFailure<Self>;

            #(#fail_rate_class)*

            fn fail_action(&self) -> Self::FailAction {
                if Self::FAIL_RATE == 0 {
                    unreachable!("Action cannot fail")
                }

                crate::actions::failure::NullFailure(*self)
            }
        }
    )
    .into()
}

enum FfxivAttr {
    Constant(LitInt),
    Kind(LitStr),
    // Name(syn::Path),
}

impl FfxivAttr {
    fn to_lit_int(&self) -> &LitInt {
        match self {
            Self::Constant(lit) => lit,
            _ => panic!("Attempt to fetch lit int from non-lit-int type"),
        }
    }

    fn to_lit_str(&self) -> &LitStr {
        match self {
            Self::Kind(lit) => lit,
            _ => panic!("Attempt to fetch lit str from non-lit-str type"),
        }
    }

    // fn to_name(&self) -> &syn::Path {
    //     match self {
    //         Self::Name(name) => name,
    //         _ => panic!("Attempt to fetch name int from non-path type"),
    //     }
    // }
}

type FfxivAttrMatcher = dyn Fn(NestedMeta) -> Option<FfxivAttr>;

fn find_attributes(
    ast: &DeriveInput,
    tag_attr: &str,
    mut criteria: HashMap<&'static str, Box<FfxivAttrMatcher>>,
) -> HashMap<&'static str, FfxivAttr> {
    let mut out = HashMap::with_capacity(criteria.len());
    let mut removed = HashSet::new();

    for meta in ast.attrs.iter().filter_map(|v| v.parse_meta().ok()) {
        if let Meta::List(list) = meta {
            if list.path.is_ident(tag_attr) {
                for nested in list.nested {
                    for (&key, val) in criteria.iter() {
                        if let Some(result) = val(nested.clone()) {
                            removed.insert(key);
                            out.insert(key, result);
                        }
                    }

                    criteria.retain(|k, _| removed.contains(k));
                    removed.clear();

                    if criteria.is_empty() {
                        return out;
                    }
                }
            }
        }
    }
    out
}

fn attr_literal(assoc_const_attr: &'static str) -> impl Fn(NestedMeta) -> Option<FfxivAttr> {
    |nested: NestedMeta| -> Option<FfxivAttr> {
        match nested {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Int(lit),
                ..
            })) if path.is_ident(assoc_const_attr) => Some(FfxivAttr::Constant(lit)),
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            })) if path.is_ident(assoc_const_attr) => Some(FfxivAttr::Kind(lit)),
            _ => None,
        }
    }
}