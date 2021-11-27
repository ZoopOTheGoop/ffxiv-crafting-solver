use std::iter;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, parse_quote, Arm, FnArg, Ident, ItemEnum, ItemFn, ItemImpl, Path,
    Receiver, TraitItemType,
};

pub fn magic_action_passthrough(input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as ItemEnum);

    let ident = target.ident;

    let idents = target
        .variants
        .iter()
        .map(|v| v.ident.clone())
        .collect::<Vec<_>>();

    let mut froms = gen_from(idents.clone().into_iter(), &ident);

    let traits = gen_traits(idents.clone().into_iter(), &ident).into_iter();

    let rand_actions = gen_rand_action(idents.into_iter(), &ident);

    let traits_enums: TokenStream = quote!(
        #(#traits)*
        #rand_actions
    )
    .into();

    froms.extend(traits_enums.into_iter());
    froms
}

struct TraitBlueprint {
    name: Path,
    assoc_type: Option<TraitItemType>,
    funcs: Vec<(ItemFn, Vec<Ident>)>,
}

fn gen_traits<I: Iterator<Item = Ident> + Clone>(variants: I, me: &Ident) -> Vec<ItemImpl> {
    let mut to_gen = vec![];
    for blueprint in traits() {
        let funcs = blueprint
            .funcs
            .into_iter()
            .map(|(head, args)| {
                (
                    gen_fn(variants.clone(), head, args.clone(), blueprint.name.clone()),
                    args,
                )
            })
            .collect();

        to_gen.push(gen_trait(
            variants.clone(),
            me.clone(),
            blueprint.name,
            blueprint.assoc_type,
            funcs,
        ));
    }

    to_gen
}

fn traits() -> Vec<TraitBlueprint> {
    vec![
        TraitBlueprint {
            name: parse_quote!(crate::actions::TimePassing),
            assoc_type: None,
            funcs: vec![(
                parse_quote!(
                    fn time_passed<C, M>(&self, state: &crate::CraftingState<C, M>) -> bool
                    where
                        C: crate::conditions::Condition,
                        M: crate::quality_map::QualityMap,
                    {
                    }
                ),
                vec![parse_quote!(state)],
            )],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::ActionLevel),
            assoc_type: None,
            funcs: vec![(
                parse_quote!(
                    fn level(&self) -> u16 {}
                ),
                vec![],
            )],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::CpCost),
            assoc_type: None,
            funcs: vec![(
                parse_quote!(
                    fn cp_cost<C, M>(&self, state: &crate::CraftingState<C, M>) -> i16
                    where
                        C: crate::conditions::Condition,
                        M: crate::quality_map::QualityMap,
                    {
                    }
                ),
                vec![parse_quote!(state)],
            )],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::DurabilityFactor),
            assoc_type: None,
            funcs: vec![(
                parse_quote!(
                    fn durability<C>(&self, buffs: &crate::buffs::BuffState, condition: &C) -> i8
                    where
                        C: crate::conditions::Condition,
                    {
                    }
                ),
                vec![parse_quote!(buffs), parse_quote!(condition)],
            )],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::CanExecute),
            assoc_type: None,
            funcs: vec![(
                parse_quote!(
                    fn can_execute<C, M>(&self, state: &crate::CraftingState<C, M>) -> bool
                    where
                        C: crate::conditions::Condition,
                        M: crate::quality_map::QualityMap,
                    {
                    }
                ),
                vec![parse_quote!(state)],
            )],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::buffs::BuffAction),
            assoc_type: None,
            funcs: vec![(
                parse_quote!(
                    fn buff<C, M>(
                        &self,
                        state: &crate::CraftingState<C, M>,
                        so_far: &mut crate::buffs::BuffState,
                    ) where
                        C: crate::conditions::Condition,
                        M: crate::quality_map::QualityMap,
                    {
                    }
                ),
                vec![parse_quote!(state), parse_quote!(so_far)],
            )],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::QualityAction),
            assoc_type: None,
            funcs: vec![
                (
                    parse_quote!(
                        fn efficiency<C, M>(&self, state: &crate::CraftingState<C, M>) -> f64
                        where
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
                (
                    parse_quote!(
                        fn quality<C, M>(&self, state: &crate::CraftingState<C, M>) -> u32
                        where
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
            ],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::ProgressAction),
            assoc_type: None,
            funcs: vec![
                (
                    parse_quote!(
                        fn efficiency<C, M>(&self, state: &crate::CraftingState<C, M>) -> f64
                        where
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
                (
                    parse_quote!(
                        fn progress<C, M>(&self, state: &crate::CraftingState<C, M>) -> u32
                        where
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
            ],
        },
        TraitBlueprint {
            name: parse_quote!(crate::actions::Action),
            assoc_type: None,
            funcs: vec![
                (
                    parse_quote!(
                        fn prospective_act<C, M>(
                            self,
                            state: &crate::CraftingState<C, M>,
                        ) -> crate::actions::ActionResult
                        where
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
                (
                    parse_quote!(
                        fn act<C, M>(
                            self,
                            state: &crate::CraftingState<C, M>,
                        ) -> crate::actions::ActionOutcome
                        where
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
                (
                    parse_quote!(
                        fn act_random<
                            R: rand::Rng,
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        >(
                            self,
                            rng: &mut R,
                            state: &crate::CraftingState<C, M>,
                        ) -> crate::actions::RollOutcome<
                            crate::actions::ActionOutcome,
                            crate::actions::ActionOutcome,
                        >
                        where
                            Self: crate::actions::RandomAction,
                        {
                        }
                    ),
                    vec![parse_quote!(rng), parse_quote!(state)],
                ),
                (
                    parse_quote!(
                        fn propective_act_random<
                            R: rand::Rng,
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        >(
                            self,
                            rng: &mut R,
                            state: &crate::CraftingState<C, M>,
                        ) -> crate::actions::RollOutcome<
                            crate::actions::ActionResult,
                            crate::actions::ActionResult,
                        >
                        where
                            Self: crate::actions::RandomAction,
                        {
                        }
                    ),
                    vec![parse_quote!(rng), parse_quote!(state)],
                ),
                (
                    parse_quote!(
                        fn prospective_act_and_fail<
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        >(
                            self,
                            state: &crate::CraftingState<C, M>,
                        ) -> [(
                            u8,
                            crate::actions::RollOutcome<
                                crate::actions::ActionResult,
                                crate::actions::ActionResult,
                            >,
                        ); 2]
                        where
                            Self: crate::actions::RandomAction,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
                (
                    parse_quote!(
                        fn act_and_fail<
                            C: crate::conditions::Condition,
                            M: crate::quality_map::QualityMap,
                        >(
                            self,
                            state: &crate::CraftingState<C, M>,
                        ) -> [(
                            u8,
                            crate::actions::RollOutcome<
                                crate::actions::ActionOutcome,
                                crate::actions::ActionOutcome,
                            >,
                        ); 2]
                        where
                            Self: crate::actions::RandomAction,
                        {
                        }
                    ),
                    vec![parse_quote!(state)],
                ),
            ],
        },
    ]
}

fn gen_from<I: Iterator<Item = Ident> + Clone>(variants: I, me: &Ident) -> TokenStream {
    let me = iter::repeat(me);
    quote!(
        #(
            #[automatically_derived]
            impl From<#variants> for #me {
                fn from(other: #variants) -> Self {
                    Self::#variants
                }
            }
        )*
    )
    .into()
}

fn gen_trait<I: Iterator<Item = Ident> + Clone>(
    variants: I,
    me: Ident,
    trait_name: Path,
    assoc_type: Option<TraitItemType>,
    funcs: Vec<(ItemFn, Vec<Ident>)>,
) -> ItemImpl {
    let fn_defs = funcs
        .into_iter()
        .map(|(func, args)| gen_fn(variants.clone(), func, args, trait_name.clone()));

    let assoc_type = assoc_type.into_iter();

    parse_quote!(
        #(#assoc_type)*

        #[automatically_derived]
        impl #trait_name for #me {
            #(#fn_defs)*
        }
    )
}

fn gen_fn<I: Iterator<Item = Ident> + Clone>(
    variants: I,
    func: ItemFn,
    args: Vec<Ident>,
    trait_name: Path,
) -> ItemFn {
    let sig = func.sig;
    let name = &sig.ident;
    let args = args.iter();
    let trait_name = trait_name;
    let self_by_ref = matches!(
        sig.receiver(),
        Some(FnArg::Receiver(Receiver {
            reference: Some(_),
            ..
        }))
    );

    // I could not figure out to make this work without manually doing this,
    //it gets mad about args being depleted after the first match, essentially
    let arms = variants.map(|variant| {
        let args = args.clone();
        let out: Arm = if self_by_ref {
            parse_quote!(Self::#variant => <#variant as #trait_name>::#name(&#variant, #(#args,)*))
        } else {
            parse_quote!(Self::#variant => <#variant as #trait_name>::#name(#variant, #(#args,)*))
        };
        out
    });

    parse_quote!(
        #sig {
            match self {
                #(#arms,)*
            }
        }
    )
}

fn gen_rand_action<I: Iterator<Item = Ident> + Clone>(variants: I, me: &Ident) -> ItemImpl {
    let all_variants = variants.clone();
    let variants = variants.filter(|v| v != "PatientTouch");
    let variants2 = variants.clone();

    parse_quote!(
        #[automatically_derived]
        impl crate::actions::RandomAction for #me {
            type FailAction = crate::actions::failure::ComboFailure<Self>;

            fn roll<R: rand::Rng, C: crate::conditions::Condition, M: crate::quality_map::QualityMap>(
                self,
                rng: &mut R,
                state: &crate::CraftingState<C, M>,
            ) -> crate::actions::RollOutcome<Self, Self::FailAction> {

                use crate::actions::failure::ComboFailure;
                use crate::actions::failure::NullFailure;
                use crate::actions::RollOutcome;

                match self {
                    Self::PatientTouch => match PatientTouch.roll(rng, state) {
                        RollOutcome::Success(_) => RollOutcome::Success(self),
                        RollOutcome::Failure(_) => RollOutcome::Failure(ComboFailure::PatientFailure)
                    },
                    #(Self::#variants => match #variants.roll(rng, state) {
                        RollOutcome::Success(_) => RollOutcome::Success(self),
                        RollOutcome::Failure(_) => RollOutcome::Failure(ComboFailure::NullFailure(NullFailure(self))),
                    },)*
                }
            }

            fn fail_rate<C: crate::conditions::Condition, M: crate::quality_map::QualityMap>(&self, state: &crate::CraftingState<C, M>) -> u8 {
                match self {
                    #(Self::#all_variants => #all_variants.fail_rate(state),)*
                }
            }

            fn fail_action(&self) -> Self::FailAction {
                use crate::actions::failure::ComboFailure;
                use crate::actions::failure::NullFailure;
                match self {
                    Self::PatientTouch => ComboFailure::PatientFailure,
                    #(Self::#variants2 => ComboFailure::NullFailure(NullFailure(*self)),)*
                }
            }
        }
    )
}
