//! This file contains basic activation tests for buff actions. While it tests some special functionality
//! (setting the `TimePassed` flag, `WasteNot2`'s special enum behavior etc), it doesn't test the long-term
//! effects of these buffs such as decay over multiple states, or the extra efficiency they apply. These are
//! more appropriate for longer aggregate trajectory tests.

use std::num::NonZeroU8;

use crate::{
    actions::{buffs::*, Action, ActionOutcome, CpCost},
    buffs::{self, DurationalBuff},
    CraftingState,
};

use super::CLASSICAL_SIMULATOR;

#[test]
fn veneration() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    let action = Veneration;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("Veneration made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute Veneration in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying veneration has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert!(
        result.curr_cp < state.curr_cp,
        "Applying veneration does not cost CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );

    let mut expected = state;
    expected.buffs.progress.veneration = buffs::progress::Veneration::default().activate(0);
    expected.curr_cp -= action.cp_cost(&state);

    assert_eq!(
        result, expected,
        "Applying veneration did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}

#[test]
fn waste_not() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    let action = WasteNot;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("Waste Not made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute Waste Not in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying Waste Not has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert!(
        result.curr_cp < state.curr_cp,
        "Applying Waste Not does not cost CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );
    assert!(matches!(
        result.buffs.durability.waste_not,
        buffs::durability::WasteNot::WasteNot(_)
    ));

    let mut expected = state;
    expected.buffs.durability.waste_not = buffs::durability::WasteNot::Inactive.activate(0);
    expected.curr_cp -= action.cp_cost(&state);

    assert_eq!(
        result, expected,
        "Applying Waste Not did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}

#[test]
fn great_strides() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    let action = GreatStrides;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("Great Strides made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute Great Strides in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying Great Strides has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert!(
        result.curr_cp < state.curr_cp,
        "Applying Great Strides does not cost CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );

    let mut expected = state;
    expected.buffs.quality.great_strides = buffs::quality::GreatStrides::default().activate(0);
    expected.curr_cp -= action.cp_cost(&state);

    assert_eq!(
        result, expected,
        "Applying Great Strides did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}

#[test]
fn innovation() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    let action = Innovation;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("Innovation made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute Innovation in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying Innovation has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert!(
        result.curr_cp < state.curr_cp,
        "Applying Innovation does not cost CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );

    let mut expected = state;
    expected.buffs.quality.innovation = buffs::quality::Innovation::default().activate(0);
    expected.curr_cp -= action.cp_cost(&state);

    assert_eq!(
        result, expected,
        "Applying Innovation did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}

#[test]
fn final_appraisal() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    let action = FinalAppraisal;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("Final Appraisal made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute Final Appraisal in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying Final Appraisal has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert!(
        result.curr_cp < state.curr_cp,
        "Applying Final Appraisal does not cost CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );
    assert!(
        !delta.time_passed,
        "Time passed after using final appraisal"
    );

    let mut expected = state;
    expected.buffs.progress.final_appraisal =
        buffs::progress::FinalAppraisal::default().activate(0);
    expected.curr_cp -= action.cp_cost(&state);

    assert_eq!(
        result, expected,
        "Applying Final Appraisal did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}

#[test]
fn waste_not_2() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    let action = WasteNot2;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("Waste Not 2 made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute Waste Not 2 in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying Waste Not 2 has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert!(
        result.curr_cp < state.curr_cp,
        "Applying Waste Not 2 does not cost CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );
    assert_eq!(
        result.buffs.durability.waste_not,
        buffs::durability::WasteNot::WasteNot2(NonZeroU8::new(8).unwrap()),
    );

    let mut expected = state;
    expected.buffs.durability.waste_not = buffs::durability::WasteNot::Inactive.activate(4);
    expected.curr_cp -= action.cp_cost(&state);

    assert_eq!(
        result, expected,
        "Applying Waste Not 2 did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}

#[test]
fn manipulation() {
    let state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    let action = Manipulation;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("Manipulation made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute Manipulation in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying Manipulation has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert!(
        result.curr_cp < state.curr_cp,
        "Applying Manipulation does not cost CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );

    let mut expected = state;
    expected.buffs.durability.manipulation = buffs::durability::Manipulation::default().activate(0);
    expected.curr_cp -= action.cp_cost(&state);

    assert_eq!(
        result, expected,
        "Applying Manipulation did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}

#[test]
fn heart_and_soul() {
    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);
    state.buffs.specialist_actions = buffs::misc::SpecialistActions::Availalble(3);
    let state = state;

    let action = HeartAndSoul;
    let outcome = action.prospective_act(&state);
    let delta = match outcome {
        Ok(outcome) => match outcome {
            ActionOutcome::InProgress(delta) => delta,
            other => panic!("HeartAndSoul made craft fail or succeed: {:?}", other),
        },
        Err(err) => panic!(
            "Error trying to execute HeartAndSoul in a valid state: {}",
            err
        ),
    };

    let result = state + delta;

    assert_ne!(
        state, result,
        "Applying HeartAndSoul has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state, result, delta
    );
    assert_eq!(
        result.curr_cp, state.curr_cp,
        "Applying HeartAndSoul costs CP (should only cost specialist actions);\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
        state,
        result,
        delta
    );

    let mut expected = state;
    expected.buffs.heart_and_soul = buffs::misc::HeartAndSoul::Active;
    expected.buffs.specialist_actions = buffs::misc::SpecialistActions::Availalble(2);

    assert_eq!(
        result, expected,
        "Applying HeartAndSoul did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}", expected, result, delta, 
    )
}
