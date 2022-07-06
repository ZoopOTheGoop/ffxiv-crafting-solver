use crate::{
    actions::{
        misc::*,
        progress::FocusedSynthesis,
        quality::{BasicTouch, FocusedTouch},
        Action, RandomAction,
    },
    buffs::{self, DurationalBuff},
    conditions::QARegularConditions,
    CraftingState,
};

use super::{ActionTester, CLASSICAL_SIMULATOR};

#[test]
fn masters_mend() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    let state = state + BasicTouch.prospective_act(&state).unwrap().outcome();

    ActionTester::make(MastersMend, "Master's Mend", Some(state))
        .had_effect()
        .modified_cp(-88)
        .passed_time(true)
        .changed_durability(10);
}

#[test]
fn masters_mend_unsaturated() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    for _ in 0..4 {
        state = state + BasicTouch.prospective_act(&state).unwrap().outcome();
    }
    let state = state;

    ActionTester::make(MastersMend, "Master's Mend", Some(state))
        .had_effect()
        .modified_cp(-88)
        .passed_time(true)
        .changed_durability(30);
}

#[test]
fn observe() {
    ActionTester::make(Observe, "Observe", None)
        .had_effect()
        .modified_cp(-7)
        .passed_time(true)
        .changed_durability(0)
        .triggered_buff(buffs::combo::ObserveCombo::Active, |buffs| {
            buffs.combo.observation
        });
}

#[test]
fn observe_combo() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    assert_eq!(
        FocusedSynthesis.fail_rate(&state),
        50,
        "Focused Synthesis fail rate wrong",
    );
    assert_eq!(
        FocusedTouch.fail_rate(&state),
        50,
        "Focused Touch fail rate wrong"
    );

    let state = state + Observe.act(&state).outcome();
    println!("{:?}", state);

    assert_eq!(
        FocusedSynthesis.fail_rate(&state),
        0,
        "Focused Synthesis does not alter fail rate on observation combo"
    );
    assert_eq!(
        FocusedTouch.fail_rate(&state),
        0,
        "Focused Touch does not alter fail rate on observation combo"
    );
}

#[test]
fn tricks_of_the_trade() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.curr_cp -= 10;
    state.condition = QARegularConditions::Good;
    let state = state;

    ActionTester::make(TricksOfTheTrade, "Tricks of the Trade", Some(state))
        .had_effect()
        .modified_cp(10)
        .passed_time(true)
        .changed_durability(0);
}

#[test]
fn tricks_of_the_trade_unsaturated() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.curr_cp -= 30;
    state.condition = QARegularConditions::Good;
    let state = state;

    ActionTester::make(TricksOfTheTrade, "Tricks of the Trade", Some(state))
        .had_effect()
        .modified_cp(20)
        .passed_time(true)
        .changed_durability(0);
}

#[test]
fn delicate_synthesis() {
    ActionTester::make(DelicateSynthesis, "Delicate Synthesis", None)
        .had_effect()
        .modified_cp(-32)
        .passed_time(true)
        .changed_durability(-10)
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .added_progress(228)
        .added_quality(252);
}

#[test]
fn careful_observation() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.specialist_actions = buffs::misc::SpecialistActions::Availalble(3);
    state.buffs.quality.innovation = buffs::quality::Innovation::default().activate(0);
    let state = state;

    ActionTester::make(CarefulObservation, "Careful Observation", Some(state))
        .had_effect()
        .modified_cp(0)
        .used_delineation()
        .passed_time(false)
        .changed_durability(0);
}

#[test]
fn careful_observation_no_buff_tick() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.specialist_actions = buffs::misc::SpecialistActions::Availalble(3);
    state.buffs.quality.innovation = buffs::quality::Innovation::default().activate(0);
    let state = state;

    let result = state + CarefulObservation.act(&state).outcome();
    assert_eq!(
        result.buffs.quality.innovation, state.buffs.quality.innovation,
        "Buff duration improperly ticked down during Careful Observation"
    );
}
