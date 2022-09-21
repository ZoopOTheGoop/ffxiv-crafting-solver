//! For complex sequences of actions, e.g. testing entire crafts

use super::{CLASSICAL_SIMULATOR, TEST_STATS};
use crate::{
    actions::{collection::FfxivCraftingActions, Action, ActionOutcome},
    conditions::QARegularConditions,
    quality_map::{HQChance, HQMap},
    recipe::{RLvl, Recipe},
    CraftingSimulator, CraftingState,
};

#[test]
fn pactmaker_classical_rotation() {
    use FfxivCraftingActions::*;
    const ACTIONS: [FfxivCraftingActions; 27] = [
        MuscleMemory,
        Veneration,
        WasteNot2,
        FinalAppraisal,
        Groundwork,
        CarefulSynthesis,
        CarefulSynthesis,
        Innovation,
        BasicTouch,
        BasicTouch,
        StandardTouch,
        AdvancedTouch,
        Manipulation,
        Innovation,
        PrudentTouch,
        PrudentTouch,
        PrudentTouch,
        PrudentTouch,
        Innovation,
        PrudentTouch,
        PrudentTouch,
        PrudentTouch,
        TrainedFinesse,
        Innovation,
        GreatStrides,
        ByregotsBlessing,
        BasicSynthesis,
    ];

    test_action_sequence(None, &ACTIONS);
}

#[test]
fn pactmaker_pactmaker_rotation_with_hq_start() {
    let pactmaker = Recipe::try_from_rlvl_modifiers(RLvl(590), 160, 100, 100).unwrap();
    let sim = CraftingSimulator::from_character_recipe(TEST_STATS, pactmaker);
    let mut state = CraftingState::new_simulation(&sim);
    state.curr_quality = 1791; // HQ Eblan Danburite + Ophiotauros Leather
    let state = state;

    use FfxivCraftingActions::*;
    const ACTIONS: [FfxivCraftingActions; 26] = [
        MuscleMemory,
        Veneration,
        WasteNot,
        Groundwork,
        CarefulSynthesis,
        CarefulSynthesis,
        PreparatoryTouch,
        Manipulation,
        Innovation,
        PrudentTouch,
        PrudentTouch,
        PrudentTouch,
        PrudentTouch,
        Innovation,
        PrudentTouch,
        BasicTouch,
        StandardTouch,
        AdvancedTouch,
        Innovation,
        TrainedFinesse,
        TrainedFinesse,
        TrainedFinesse,
        Innovation,
        GreatStrides,
        ByregotsBlessing,
        CarefulSynthesis,
    ];

    test_action_sequence(Some(state), &ACTIONS);
}

fn test_action_sequence(
    initial_state: Option<CraftingState<QARegularConditions, HQMap>>,
    actions: &[FfxivCraftingActions],
) {
    let mut state =
        initial_state.unwrap_or_else(|| CraftingState::new_simulation(&CLASSICAL_SIMULATOR));

    for (i, action) in actions.iter().take(actions.len() - 1).enumerate() {
        let outcome = action.act(&state);
        assert!(
            matches!(outcome, ActionOutcome::InProgress(_)),
            "Outcome fails or completes in middle of rotation on action {} ({:?});\n\tOutcome: {:?}\n\tState: {:?}",
            i,
            action,
            outcome,
            state,
        );
        state += outcome.outcome();
    }

    let outcome = actions[actions.len() - 1].act(&state);

    assert!(
        matches!(outcome, ActionOutcome::Completed(_)),
        "Outcome is not completed after known good macro;\n\tOutcome: {:?}\n\tState: {:?}",
        outcome,
        state
    );
    assert_eq!(
        outcome.map_quality(&state),
        Some(HQChance(100)),
        "Outcome is not HQ 100% in known good rotation {:?}",
        outcome
    );
}
