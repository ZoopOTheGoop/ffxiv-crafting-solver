//! Defines the effects of progress-related actions.

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

/// An action that has some effect on the `progress` attribute. The
/// [`EFFICIENCY`](ProgressAction::EFFICIENCY) is the base efficiency of the given
/// action, without any modifiers.
pub trait ProgressAction {
    const EFFICIENCY: u16;

    /// Calculates the efficiency of the current action on the crafting state. By default this is simply the efficiency bonus granted buffs,
    /// multiplied by the action's efficiency.
    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        let efficiency_mod = 100. + state.buffs.progress.efficiency_mod() as f64 / 100.;

        efficiency_mod * Self::EFFICIENCY as f64
    }

    /// Returns the amount of progress that will be added by executing the given `action` in the current `state`.
    ///
    /// Takes into account [`Condition`] modifiers as well as any [progress buffs].
    ///
    /// [progress buffs]: crate::buffs::progress
    fn progress<C, M>(&self, state: &CraftingState<C, M>) -> u32
    where
        C: Condition,
        M: QualityMap,
    {
        let progress = state.base_progress();
        let condition_mod = state.condition.to_progress_modifier() as u64 as f64 / 100.;
        let efficiency = self.efficiency(state);

        (progress.floor() * condition_mod * efficiency) as u32
    }
}
