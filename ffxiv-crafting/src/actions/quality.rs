//! Defines the effects on quality that actions have, as well as collects actions whose primary purpose is increasing the quality property.

use crate::{
    buffs::quality::InnerQuiet, conditions::Condition, quality_map::QualityMap, CraftingState,
};

pub use self::concrete::*;

/// An action's effect on the `quality` attribute. The
/// [`EFFICIENCY`](QualityAction::EFFICIENCY) is the base efficiency of the given
/// action, without any modifiers.
pub trait QualityAction {
    const EFFICIENCY: u16 = 0;

    /// Applies the tick of IQ that's added before a quality action executes.
    ///
    /// Further buffs to IQ (e.g. from [`PreciseTouch`] are applies in the normal buff stage).
    fn pre_iq(&self, iq: &mut InnerQuiet) {
        if Self::EFFICIENCY == 0 {
            return;
        }
        *iq += 1;
    }

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
    fn quality<C, M>(&self, state: &CraftingState<C, M>, iq: &mut InnerQuiet) -> u32
    where
        C: Condition,
        M: QualityMap,
    {
        if Self::EFFICIENCY == 0 {
            return 0;
        }

        self.pre_iq(iq);

        let quality = state.base_quality();
        let condition_mod = state.condition.to_quality_modifier() as u64 as f64 / 100.;
        let efficiency = self.efficiency(state);

        ((quality * condition_mod).floor() * efficiency) as u32
    }
}

mod concrete {
    use ffxiv_crafting_derive::{
        ActionLevel, BuffAction, CanExecute, CpCost, DurabilityFactor, ProgressAction,
        QualityAction, RandomAction,
    };

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction)]
    #[ffxiv_cp(cost = 18)]
    #[ffxiv_quality(efficiency = 100)]
    #[ffxiv_act_lvl(level = 5)]
    pub struct BasicTouch;
}
