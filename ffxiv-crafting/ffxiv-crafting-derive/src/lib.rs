#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

mod actions;
mod buff;
mod condition;
mod passthrough;

#[proc_macro_derive(ProgressAction, attributes(ffxiv_progress))]
pub fn progress_action_macro_derive(input: TokenStream) -> TokenStream {
    actions::progress_action(input)
}

#[proc_macro_derive(QualityAction, attributes(ffxiv_quality))]
pub fn quality_action_macro_derive(input: TokenStream) -> TokenStream {
    actions::quality_action(input)
}

#[proc_macro_derive(CpCost, attributes(ffxiv_cp))]
pub fn cp_cost_macro_derive(input: TokenStream) -> TokenStream {
    actions::cp_cost(input)
}

#[proc_macro_derive(DurabilityFactor, attributes(ffxiv_durability))]
pub fn durability_factor_macro_derive(input: TokenStream) -> TokenStream {
    actions::durability(input)
}

#[proc_macro_derive(CanExecute, attributes(ffxiv_can_exe))]
pub fn can_execute_macro_derive(input: TokenStream) -> TokenStream {
    actions::can_execute(input)
}

#[proc_macro_derive(BuffAction, attributes(ffxiv_buff_act))]
pub fn buff_action(input: TokenStream) -> TokenStream {
    actions::buff_action(input)
}

#[proc_macro_derive(ActionLevel, attributes(ffxiv_act_lvl))]
pub fn action_level(input: TokenStream) -> TokenStream {
    actions::action_level(input)
}

#[proc_macro_derive(RandomAction, attributes(ffxiv_rand_act))]
pub fn random_action(input: TokenStream) -> TokenStream {
    actions::random_action(input)
}

#[proc_macro_derive(TimePassing, attributes(ffxiv_no_time_pass))]
pub fn time_passed(input: TokenStream) -> TokenStream {
    actions::time_passed(input)
}

#[proc_macro_derive(Action)]
pub fn action(input: TokenStream) -> TokenStream {
    actions::action(input)
}

#[proc_macro_derive(PassthroughAction)]
pub fn ffxiv_action_enum(input: TokenStream) -> TokenStream {
    passthrough::magic_action_passthrough(input)
}

#[proc_macro_derive(Condition, attributes(ffxiv))]
pub fn condition_macro_derive(input: TokenStream) -> TokenStream {
    condition::condition_derive(input)
}

#[proc_macro_derive(Buff)]
pub fn buff_derive(input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as ItemStruct);

    let ident = target.ident;
    quote! (
        #[automatically_derived]
        impl Buff for #ident {
            fn is_active(&self) -> bool {
                return self.0 > 0
            }
        }
    )
    .into()
}

#[proc_macro_derive(DurationalBuff, attributes(ffxiv))]
pub fn durational_buff_derive(input: TokenStream) -> TokenStream {
    buff::durational_buff_derive(input)
}

#[proc_macro_derive(ConsumableBuff)]
pub fn consumable_buff_derive(input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as ItemStruct);

    let ident = target.ident;
    quote! (
        #[automatically_derived]
        impl ConsumableBuff for #ident {
            fn deactivate(self) -> (Self, u8) {
                debug_assert_ne!(self.0, 0, "Attempt to deactivate inactive {}", stringify!(#ident));

                (Self(0), self.0)
            }
        }
    )
    .into()
}
