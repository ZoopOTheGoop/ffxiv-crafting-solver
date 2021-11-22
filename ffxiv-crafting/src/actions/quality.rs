//! Defines the effects of quality-related actions.

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

/// An action that has some effect on the `quality` attribute. The
/// [`EFFICIENCY`](QualityAction::EFFICIENCY) is the base efficiency of the given
/// action, without any modifiers.
pub trait QualityAction {
    const EFFICIENCY: u16 = 0;

    /// Calculates the efficiency of the current action on the crafting state. By default this is simply the efficiency bonus granted buffs,
    /// multiplied by the action's efficiency.
    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        if Self::EFFICIENCY == 0 {
            return 0.;
        }
        let efficiency_mod = 100. + state.buffs.quality.efficiency_mod() as f64 / 100.;

        efficiency_mod * Self::EFFICIENCY as f64
    }

    /// Returns the amount of quality that will be added by executing the given `action` in the current `state`.
    ///
    /// Takes into account [`Condition`] modifiers as well as any [quality buffs].
    ///
    /// [quality buffs]: crate::buffs::quality
    fn quality<C, M>(&self, state: &CraftingState<C, M>) -> u32
    where
        C: Condition,
        M: QualityMap,
    {
        if Self::EFFICIENCY == 0 {
            return 0;
        }

        let quality = state.base_quality();
        let condition_mod = state.condition.to_quality_modifier() as u64 as f64 / 100.;
        let efficiency = self.efficiency(state);

        ((quality * condition_mod).floor() * efficiency) as u32
    }
}
