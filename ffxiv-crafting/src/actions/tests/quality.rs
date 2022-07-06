use crate::{
    actions::{quality::*, Action},
    buffs::{self},
    conditions::QARegularConditions,
    CraftingState,
};

use super::{ActionTester, CLASSICAL_SIMULATOR};

#[test]
fn basic_touch() {
    ActionTester::make(BasicTouch, "Basic Touch", None)
        .had_effect()
        .modified_cp(-18)
        .passed_time(true)
        .triggered_buff(buffs::combo::BasicTouchCombo::BasicTouch, |buffs| {
            buffs.combo.basic_touch
        })
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_quality(252);
}

#[test]
fn hasty_touch() {
    // Note, this is assuming it succeeds, we'll test probability stuff elsewhere and the probability in codegen tests

    ActionTester::make(HastyTouch, "Hasty Touch", None)
        .had_effect()
        .modified_cp(0)
        .passed_time(true)
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_quality(252);
}

#[test]
fn standard_touch() {
    ActionTester::make(StandardTouch, "Standard Touch", None)
        .had_effect()
        .modified_cp(-32)
        .passed_time(true)
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        // Technically tests that the combo (properly) does NOT trigger - combo tests are done elsewhere
        .triggered_buff(buffs::combo::BasicTouchCombo::Inactive, |buffs| {
            buffs.combo.basic_touch
        })
        .changed_durability(-10)
        .added_quality(315);
}

#[test]
fn byregots_blessing() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    // sets up an IQ stack in a real scenario, we can't get a valid number otherwise
    let state = state + BasicTouch.prospective_act(&state).unwrap().outcome();

    ActionTester::make(ByregotsBlessing, "Byregot's Blessing", Some(state))
        .had_effect()
        .modified_cp(-24)
        .passed_time(true)
        // Should consume IQ
        .triggered_buff(buffs::quality::InnerQuiet::default(), |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_quality(332);
}

#[test]
fn precise_touch() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.condition = QARegularConditions::Good;
    let state = state;

    ActionTester::make(PreciseTouch, "Precise Touch", Some(state))
        .had_effect()
        .modified_cp(-18)
        .passed_time(true)
        // Should consume IQ
        .triggered_buff(buffs::quality::InnerQuiet::default() + 2, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_quality(567); // Part of this is due to condition modifier
}

#[test]
#[should_panic(expected = "Cannot execute this action in the current state")]
fn precise_touch_normal() {
    ActionTester::make(PreciseTouch, "Precise Touch", None);
}

#[test]
fn prudent_touch() {
    ActionTester::make(PrudentTouch, "Prudent Touch", None)
        .had_effect()
        .modified_cp(-25)
        .passed_time(true)
        // Should consume IQ
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-5)
        .added_quality(252);
}

#[test]
fn focused_touch() {
    ActionTester::make(FocusedTouch, "Focused Touch", None)
        .had_effect()
        .modified_cp(-18)
        .passed_time(true)
        // Should consume IQ
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_quality(378);
}

#[test]
fn reflect() {
    ActionTester::make(Reflect, "Reflect", None)
        .had_effect()
        .modified_cp(-6)
        .passed_time(true)
        // Should consume IQ
        .triggered_buff(buffs::quality::InnerQuiet::default() + 2, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_quality(252);
}
