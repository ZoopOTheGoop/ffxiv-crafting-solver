use crate::{
    actions::{buffs::WasteNot, progress::*, quality::BasicTouch, Action, ActionOutcome},
    buffs::{self, DurationalBuff},
    conditions::QARegularConditions,
    CraftingState,
};

use super::{ActionTester, CLASSICAL_SIMULATOR};

#[test]
fn basic_synthesis() {
    ActionTester::make(BasicSynthesis, "Basic Synthesis", None)
        .had_effect()
        .modified_cp(0)
        .passed_time(true)
        // Just for one synthesis action, make sure IQ isn't triggered
        .triggered_buff(buffs::quality::InnerQuiet::default(), |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_progress(273)
        // Again, just for one synthesis action, make sure quality isn't affected
        .added_quality(0);
}

#[test]
fn rapid_synthesis() {
    ActionTester::make(RapidSynthesis, "Rapid Synthesis", None)
        .had_effect()
        .modified_cp(0)
        .passed_time(true)
        .changed_durability(-10)
        .added_progress(1140);
}

#[test]
fn muscle_memory() {
    ActionTester::make(MuscleMemory, "Muscle Memory", None)
        .had_effect()
        .modified_cp(-6)
        .passed_time(true)
        .changed_durability(-10)
        .added_progress(684)
        .triggered_buff(
            buffs::progress::MuscleMemory::default().activate(0),
            |buffs| buffs.progress.muscle_memory,
        );
}

#[test]
#[should_panic(expected = "Cannot execute this action in the current state")]
fn muscle_memory_first_step() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    let state = state + BasicTouch.prospective_act(&state).unwrap().outcome();

    ActionTester::make(MuscleMemory, "Muscle Memory", Some(state));
}

#[test]
fn careful_synthesis() {
    ActionTester::make(CarefulSynthesis, "Careful Synthesis", None)
        .had_effect()
        .modified_cp(-7)
        .passed_time(true)
        .changed_durability(-10)
        .added_progress(410);
}

#[test]
fn focused_synthesis() {
    ActionTester::make(FocusedSynthesis, "Focused Synthesis", None)
        .had_effect()
        .modified_cp(-5)
        .passed_time(true)
        .changed_durability(-10)
        .added_progress(456);
}

#[test]
fn groundwork() {
    ActionTester::make(Groundwork, "Groundwork", None)
        .had_effect()
        .modified_cp(-18)
        .passed_time(true)
        .changed_durability(-20)
        .added_progress(820);
}

#[test]
fn groundwork_durability_efficiency() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.curr_durability = 15;
    let state = state;

    let outcome = Groundwork.prospective_act(&state).unwrap();
    assert!(
        matches!(outcome, ActionOutcome::Failure(_)),
        "Groundwork at low durability did not make craft fail"
    );
    let delta = outcome.outcome();

    assert_eq!(
        delta.added_progress, 410,
        "Groundwork efficiency not halved under low durability conditions"
    );
}

#[test]
fn intensive_synthesis() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.condition = QARegularConditions::Good;
    let state = state;

    ActionTester::make(IntensiveSynthesis, "Intensive Synthesis", Some(state))
        .had_effect()
        .modified_cp(-6)
        .passed_time(true)
        .changed_durability(-10)
        .added_progress(912);
}

#[test]
#[should_panic(expected = "Cannot execute this action in the current state")]
fn intensive_synthesis_normal() {
    ActionTester::make(IntensiveSynthesis, "Intensive Synthesis", None);
}

#[test]
fn prudent_synthesis() {
    ActionTester::make(PrudentSynthesis, "Prudent Synthesis", None)
        .had_effect()
        .modified_cp(-18)
        .passed_time(true)
        .changed_durability(-5)
        .added_progress(410);
}

#[test]
#[should_panic(expected = "Cannot execute this action in the current state")]
fn prudent_synthesis_waste_not() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    let state = state + WasteNot.prospective_act(&state).unwrap().outcome();

    ActionTester::make(PrudentSynthesis, "Prudent Synthesis", Some(state));
}
