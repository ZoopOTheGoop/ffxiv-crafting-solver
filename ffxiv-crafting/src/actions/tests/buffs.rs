//! This file contains basic activation tests for buff actions. While it tests some special functionality
//! (setting the `TimePassed` flag, `WasteNot2`'s special enum behavior etc), it doesn't test the long-term
//! effects of these buffs such as decay over multiple states, or the extra efficiency they apply. These are
//! more appropriate for longer aggregate trajectory tests.

use std::num::NonZeroU8;

use crate::{
    actions::{buffs::*, CpCost},
    buffs::{self, DurationalBuff},
    CraftingState,
};

use super::{ActionTester, CLASSICAL_SIMULATOR};

#[test]
fn veneration() {
    ActionTester::make(Veneration, "Veneration", None)
        .had_effect()
        .used_cp(Veneration::CP_COST)
        .passed_time(true)
        .triggered_buff(
            buffs::progress::Veneration::default().activate(0),
            |buffs| buffs.progress.veneration,
        );
}

#[test]
fn waste_not() {
    ActionTester::make(WasteNot, "Waste Not", None)
        .had_effect()
        .used_cp(WasteNot::CP_COST)
        .passed_time(true)
        .triggered_buff(
            // Be explicit on this one to check the right variant
            buffs::durability::WasteNot::WasteNot(NonZeroU8::new(4).unwrap()),
            |buffs| buffs.durability.waste_not,
        );
}

#[test]
fn great_strides() {
    ActionTester::make(GreatStrides, "Great Strides", None)
        .had_effect()
        .used_cp(GreatStrides::CP_COST)
        .passed_time(true)
        .triggered_buff(
            buffs::quality::GreatStrides::default().activate(0),
            |buffs| buffs.quality.great_strides,
        );
}

#[test]
fn innovation() {
    ActionTester::make(Innovation, "Innovation", None)
        .had_effect()
        .used_cp(Innovation::CP_COST)
        .passed_time(true)
        .triggered_buff(buffs::quality::Innovation::default().activate(0), |buffs| {
            buffs.quality.innovation
        });
}

#[test]
fn final_appraisal() {
    ActionTester::make(FinalAppraisal, "Final Appraisal", None)
        .had_effect()
        .used_cp(FinalAppraisal::CP_COST)
        .passed_time(false)
        .triggered_buff(
            buffs::progress::FinalAppraisal::default().activate(0),
            |buffs| buffs.progress.final_appraisal,
        );
}

#[test]
fn waste_not_2() {
    ActionTester::make(WasteNot2, "Waste Not 2", None)
        .had_effect()
        .used_cp(WasteNot2::CP_COST)
        .passed_time(true)
        .triggered_buff(
            // Be explicit on this one to check the right variant
            buffs::durability::WasteNot::WasteNot2(NonZeroU8::new(8).unwrap()),
            |buffs| buffs.durability.waste_not,
        );
}

#[test]
fn manipulation() {
    ActionTester::make(Manipulation, "Manipulation", None)
        .had_effect()
        .used_cp(Manipulation::CP_COST)
        .passed_time(true)
        .triggered_buff(
            // Be explicit on this one to check the right variant
            buffs::durability::Manipulation::default().activate(0),
            |buffs| buffs.durability.manipulation,
        );
}

#[test]
fn heart_and_soul() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.specialist_actions = buffs::misc::SpecialistActions::Availalble(3);
    let state = state;

    ActionTester::make(HeartAndSoul, "Heart And Soul", Some(state))
        .had_effect()
        .used_cp(0)
        .used_delineation()
        .passed_time(false)
        .triggered_buff(buffs::misc::HeartAndSoul::Active, |buffs| {
            buffs.heart_and_soul
        });
}
