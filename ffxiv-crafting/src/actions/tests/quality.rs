use crate::{
    actions::{quality::*, Action, CpCost, DurabilityFactor},
    buffs::{self},
    CraftingState,
};

use super::{ActionTester, CLASSICAL_SIMULATOR};

#[test]
fn basic_touch() {
    ActionTester::make(BasicTouch, "Basic Touch", None)
        .had_effect()
        .used_cp(BasicTouch::CP_COST)
        .passed_time(true)
        .triggered_buff(buffs::combo::BasicTouchCombo::BasicTouch, |buffs| {
            buffs.combo.basic_touch
        })
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(BasicTouch::DURABILITY_USAGE)
        .added_quality(252);
}

#[test]
fn hasty_touch() {
    // Note, this is assuming it succeeds, we'll test probability stuff elsewhere and the probability in codegen tests

    ActionTester::make(HastyTouch, "Hasty Touch", None)
        .had_effect()
        .used_cp(HastyTouch::CP_COST)
        .passed_time(true)
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(HastyTouch::DURABILITY_USAGE)
        .added_quality(252);
}

#[test]
fn standard_touch() {
    ActionTester::make(StandardTouch, "Standard Touch", None)
        .had_effect()
        .used_cp(StandardTouch::CP_COST)
        .passed_time(true)
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        // Technically tests that the combo (properly) does NOT trigger - combo tests are done elsewhere
        .triggered_buff(buffs::combo::BasicTouchCombo::Inactive, |buffs| {
            buffs.combo.basic_touch
        })
        .changed_durability(StandardTouch::DURABILITY_USAGE)
        .added_quality(315);
}

#[test]
fn byregots_blessing() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    // sets up an IQ stack in a real scenario, we can't get a valid number otherwise
    let state = state + BasicTouch.prospective_act(&state).unwrap().outcome();

    ActionTester::make(ByregotsBlessing, "Byregot's Blessing", Some(state))
        .had_effect()
        .used_cp(ByregotsBlessing::CP_COST)
        .passed_time(true)
        // Should consume IQ
        .triggered_buff(buffs::quality::InnerQuiet::default(), |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(ByregotsBlessing::DURABILITY_USAGE)
        .added_quality(332);
}
