//! This file contains basic activation tests for buff actions. While it tests some special functionality
//! (setting the `TimePassed` flag, `WasteNot2`'s special enum behavior etc), it doesn't test the long-term
//! effects of these buffs such as decay over multiple states, or the extra efficiency they apply. These are
//! more appropriate for longer aggregate trajectory tests.

use std::num::NonZeroU8;

use crate::{
    actions::{
        buffs::*,
        misc::TricksOfTheTrade,
        progress::{Groundwork, IntensiveSynthesis},
        quality::{BasicTouch, PreciseTouch, PrudentTouch},
        Action, ActionOutcome,
    },
    buffs::{self, DurationalBuff},
    conditions::QARegularConditions,
    CraftingState,
};

use super::{ActionTester, CLASSICAL_SIMULATOR};

#[test]
fn veneration() {
    ActionTester::make(Veneration, "Veneration", None)
        .had_effect()
        .modified_cp(-18)
        .passed_time(true)
        .triggered_buff(
            buffs::progress::Veneration::default().activate(0),
            |buffs| buffs.progress.veneration,
        )
        .changed_durability(0);
}

#[test]
fn waste_not() {
    ActionTester::make(WasteNot, "Waste Not", None)
        .had_effect()
        .modified_cp(-56)
        .passed_time(true)
        .triggered_buff(
            // Be explicit on this one to check the right variant
            buffs::durability::WasteNot::WasteNot(NonZeroU8::new(4).unwrap()),
            |buffs| buffs.durability.waste_not,
        )
        .changed_durability(0);
}

#[test]
fn waste_not_durability() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    let state = state + WasteNot.prospective_act(&state).unwrap().outcome();

    ActionTester::make(BasicTouch, "Basic Touch", Some(state)).changed_durability(-5);
    ActionTester::make(Groundwork, "Groundwork", Some(state)).changed_durability(-10);
}

#[test]
fn great_strides() {
    ActionTester::make(GreatStrides, "Great Strides", None)
        .had_effect()
        .modified_cp(-32)
        .passed_time(true)
        .triggered_buff(
            buffs::quality::GreatStrides::default().activate(0),
            |buffs| buffs.quality.great_strides,
        )
        .changed_durability(0);
}

#[test]
fn innovation() {
    ActionTester::make(Innovation, "Innovation", None)
        .had_effect()
        .modified_cp(-18)
        .passed_time(true)
        .triggered_buff(buffs::quality::Innovation::default().activate(0), |buffs| {
            buffs.quality.innovation
        })
        .changed_durability(0);
}

#[test]
fn final_appraisal() {
    ActionTester::make(FinalAppraisal, "Final Appraisal", None)
        .had_effect()
        .modified_cp(-1)
        .passed_time(false)
        .triggered_buff(
            buffs::progress::FinalAppraisal::default().activate(0),
            |buffs| buffs.progress.final_appraisal,
        )
        .changed_durability(0);
}

#[test]
fn waste_not_2() {
    ActionTester::make(WasteNot2, "Waste Not 2", None)
        .had_effect()
        .modified_cp(-98)
        .passed_time(true)
        .triggered_buff(
            // Be explicit on this one to check the right variant
            buffs::durability::WasteNot::WasteNot2(NonZeroU8::new(8).unwrap()),
            |buffs| buffs.durability.waste_not,
        )
        .changed_durability(0);
}

#[test]
fn waste_not_2_durability() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    let state = state + WasteNot2.prospective_act(&state).unwrap().outcome();

    ActionTester::make(BasicTouch, "Basic Touch", Some(state)).changed_durability(-5);
    ActionTester::make(Groundwork, "Groundwork", Some(state)).changed_durability(-10);
}

#[test]
fn manipulation() {
    ActionTester::make(Manipulation, "Manipulation", None)
        .had_effect()
        .modified_cp(-96)
        .passed_time(true)
        .triggered_buff(
            // Be explicit on this one to check the right variant
            buffs::durability::Manipulation::default().activate(0),
            |buffs| buffs.durability.manipulation,
        )
        .changed_durability(0);
}

#[test]
fn manipulation_does_not_repair_when_applied() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.curr_durability -= 20;

    ActionTester::make(Manipulation, "Manipulation", None)
        .had_effect()
        .modified_cp(-96)
        .passed_time(true)
        .triggered_buff(
            // Be explicit on this one to check the right variant
            buffs::durability::Manipulation::default().activate(0),
            |buffs| buffs.durability.manipulation,
        )
        .changed_durability(0);
}

#[test]
fn manipulation_durability() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.curr_durability -= 20;
    let state = state + Manipulation.act(&state).outcome();

    ActionTester::make(BasicTouch, "Basic Touch", Some(state)).changed_durability(-5);
    ActionTester::make(PrudentTouch, "Prudent Touch", Some(state)).changed_durability(0);
    ActionTester::make(Innovation, "Innovation", Some(state)).changed_durability(5);
}

#[test]
fn manipulation_does_not_fix_broken_items() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.curr_durability = 10;
    let state = state + Manipulation.act(&state).outcome();

    assert!(
        matches!(BasicTouch.act(&state), ActionOutcome::Failure(_)),
        "Manipulation prevents a failed craft from failing"
    );
}

#[test]
fn heart_and_soul() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.specialist_actions = buffs::misc::SpecialistActions::Availalble(3);
    let state = state;

    ActionTester::make(HeartAndSoul, "Heart And Soul", Some(state))
        .had_effect()
        .modified_cp(0)
        .used_delineation()
        .passed_time(false)
        .triggered_buff(buffs::misc::HeartAndSoul::Active, |buffs| {
            buffs.heart_and_soul
        })
        .changed_durability(0);
}

#[test]
fn heart_and_soul_consumed() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.heart_and_soul = buffs::misc::HeartAndSoul::Active;
    let state = state;

    ActionTester::make(PreciseTouch, "Precise Touch", Some(state))
        .triggered_buff(buffs::misc::HeartAndSoul::Inactive, |buffs| {
            buffs.heart_and_soul
        });

    ActionTester::make(IntensiveSynthesis, "Intensive Synthesis", Some(state))
        .triggered_buff(buffs::misc::HeartAndSoul::Inactive, |buffs| {
            buffs.heart_and_soul
        });

    ActionTester::make(TricksOfTheTrade, "Tricks of the Trade", Some(state))
        .triggered_buff(buffs::misc::HeartAndSoul::Inactive, |buffs| {
            buffs.heart_and_soul
        });
}

#[test]
fn heart_and_soul_condition_blocks_consumption() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.heart_and_soul = buffs::misc::HeartAndSoul::Active;
    state.condition = QARegularConditions::Good;
    let state = state;

    ActionTester::make(PreciseTouch, "Precise Touch", Some(state))
        .triggered_buff(buffs::misc::HeartAndSoul::Active, |buffs| {
            buffs.heart_and_soul
        });

    ActionTester::make(IntensiveSynthesis, "Intensive Synthesis", Some(state))
        .triggered_buff(buffs::misc::HeartAndSoul::Active, |buffs| {
            buffs.heart_and_soul
        });

    ActionTester::make(TricksOfTheTrade, "Tricks of the Trade", Some(state))
        .triggered_buff(buffs::misc::HeartAndSoul::Active, |buffs| {
            buffs.heart_and_soul
        });
}

#[test]
fn verify_buff_tick() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.quality.innovation = buffs::quality::Innovation::default().activate(0);
    let state = state;

    let result = state + BasicTouch.act(&state).outcome();
    assert_eq!(
        result.buffs.quality.innovation,
        state.buffs.quality.innovation.decay(),
        "Buff duration did not decay down"
    );
}
