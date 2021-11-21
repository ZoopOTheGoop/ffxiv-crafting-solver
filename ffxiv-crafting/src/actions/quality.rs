//! Defines the effects of quality-related actions.

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

use crate::buffs::quality::QualityEfficiencyMod;

/// An action that has some effect on the `quality` attribute. The
/// [`EFFICIENCY`](QualityAction::EFFICIENCY) is the base efficiency of the given
/// action, without any modifiers.
pub trait QualityAction: Copy {
    const EFFICIENCY: u16;

    /// A convenience method for returning efficiency without having to go through
    /// `<Self as QualityAction>::EFFICIENCY` where that's inconvenient.
    fn quality_efficiency(&self) -> u16 {
        Self::EFFICIENCY
    }

    /// Applies this quality action to `state` yielding the amount of quality that would
    /// be added by taking this action. The default impl defers to [`apply_quality`], and
    /// probably shouldn't be overridden.
    fn apply<C, M>(&self, state: &CraftingState<C, M>) -> u32
    where
        C: Condition,
        M: QualityMap,
    {
        apply_quality(state, self)
    }
}

/// Returns the amount of quality that will be added by executing the given `action` in the current `state`.
///
/// Takes into account [`Condition`] modifiers as well as any [quality buffs].
///
/// [quality buffs]: crate::buffs::quality
pub fn apply_quality<C, M, A>(state: &CraftingState<C, M>, _action: &A) -> u32
where
    C: Condition,
    M: QualityMap,
    A: QualityAction,
{
    let quality_buffs = state.quality_buffs;
    let iq = quality_buffs
        .inner_quiet
        .quality_mod(state.recipe.character.control);

    let rlvl = state.recipe.recipe.recipe_level;
    let clvl = state.recipe.character.clvl;

    let quality = iq * 35. / 100. + 35.;
    let quality = quality * (iq + 10_000.) / (rlvl.to_recipe_level_control() as f64 + 10_000.);
    let quality = quality * rlvl.to_quality_level_mod(clvl) as f64 / 100.;

    let condition_mod = state.condition.to_quality_modifier() as u64 as f64 / 100.;

    let efficiency_mod = (100.
        + quality_buffs.great_strides.efficiency_mod() as f64
        + quality_buffs.innovation.efficiency_mod() as f64)
        / 100.;

    let efficiency = efficiency_mod * A::EFFICIENCY as f64;

    ((quality * condition_mod).floor() * efficiency) as u32
}
