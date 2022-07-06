//! For complex sequences of actions, e.g. testing entire crafts

use super::CLASSICAL_SIMULATOR;
use crate::{
    actions::{collection::FfxivCraftingActions, Action, ActionOutcome},
    quality_map::HQChance,
    CraftingState,
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

    let mut state = CraftingState::new_simulation(&CLASSICAL_SIMULATOR);

    for (i, action) in ACTIONS.iter().take(ACTIONS.len() - 1).enumerate() {
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

    let outcome = ACTIONS[ACTIONS.len() - 1].act(&state);

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
