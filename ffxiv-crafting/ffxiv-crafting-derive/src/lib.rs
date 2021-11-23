//! Implements proc macros for the crafting simulator. Making these general is a non-goal and only need to work
//! in the crate itself. Most of the macros are meant to make actions much easier to generate without having to
//! manually implement traits in most cases.
//!
//! The line between when to put a generation option in here or not is fairly arbitrary, and largely just comes down
//! to how often you'll do the same thing without it and how easy it is to add an extension.
//!
//! Arguably deriving most of the crafting buffs could be part of the buff derive, but they're not because they're
//! one line and just annoying enough to derive that it's easier to just manually implement buff actions except for
//! very boilerplatey cases like "touch" actions affecting IQ.

extern crate proc_macro;

use proc_macro::TokenStream;

mod actions;
mod condition;

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

#[proc_macro_derive(Condition, attributes(ffxiv))]
pub fn condition_macro_derive(input: TokenStream) -> TokenStream {
    condition::condition_derive(input)
}
