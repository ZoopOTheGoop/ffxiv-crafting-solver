//! Contains miscellaneous action defitions that don't neatly fit into the other categories.

use ffxiv_crafting_derive::*;

use crate::buffs::ConsumableBuff;

use super::{buffs::BuffAction, CanExecute};

/// Spends 88 CP to instantly repair 30 durability.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, BuffAction)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 88)]
#[ffxiv_act_lvl(level = 7)]
#[ffxiv_durability(bonus = 30)]
pub struct MastersMend;

/// Uses 7 CP to pass a turn, letting buffs tick down and the current condition cycle.
///
/// Also enables [`FocusedSynthesis`] and [`PatientTouch`] to activate with 100% probability
/// if they're used as the next action.
///
/// [`FocusedSynthesis`]: crate::actions::progress::FocusedSynthesis
/// [`PatientTouch`]: crate::actions::quality::PatientTouch
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, BuffAction, Action)]
#[ffxiv_cp(cost = 7)]
#[ffxiv_act_lvl(level = 13)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_buff_act(activate = "combo.observation")]
pub struct Observe;

/// "Consumes" a [`Good`] or [`Excellent`] [`Condition`] to restore 20 CP.
///
/// [`Condition`]: crate::conditions::Condition
/// [`Good`]: crate::conditions::QARegularConditions::Good
/// [`Excellent`]: crate::conditions::QARegularConditions::Excellent
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(bonus = 20)]
#[ffxiv_act_lvl(level = 13)]
#[ffxiv_can_exe(class = "good_excellent")]
pub struct TricksOfTheTrade;

impl BuffAction for TricksOfTheTrade {
    fn deactivate_buff<C, M>(
        &self,
        state: &crate::CraftingState<C, M>,
        so_far: &mut crate::buffs::BuffState,
    ) where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        if !(state.condition.is_excellent() || state.condition.is_good()) {
            so_far.heart_and_soul.deactivate_in_place();
        }
    }
}

/// A relatively expensive action that's both a touch and a synthesis. In terms of strength:
/// for quality it's as strong as [`BasicTouch`] (without its combo effect),
/// but for progress 20 efficiency weaker than [`BasicSynthesis`].
///
/// [`BasicTouch`]: crate::actions::quality::BasicTouch
/// [`BasicSynthesis`]: crate::actions::progress::BasicSynthesis
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, BuffAction)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 32)]
#[ffxiv_act_lvl(level = 76)]
#[ffxiv_quality(efficiency = 100)]
#[ffxiv_progress(efficiency = 100)]
#[ffxiv_buff_act(touch, synthesis)]
pub struct DelicateSynthesis;

/// The only "specialist" action in the game. Per craft, if you're a specialist
/// up to three crafters delineations can be used (denoted in this library
/// by [`SpecialistActions`]), and will act somewhat like a combination of
/// [`Observe`] and [`FinalAppraisal`].
///
/// It's an effective "no-op" like [`Observe`], but doesn't let time "tick" like
/// [`FinalAppraisal`], meaning it can be used to cycle conditions without
/// buffs ticking down.
///
/// [`SpecialistActions`]: crate::buffs::misc::SpecialistActions
/// [`FinalAppraisal`]: crate::actions::buffs::FinalAppraisal
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 0)]
#[ffxiv_act_lvl(level = 55)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_no_time_pass]
pub struct CarefulObservation;

impl BuffAction for CarefulObservation {
    fn buff<C, M>(&self, _: &crate::CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        so_far.specialist_actions -= 1;
    }
}

impl CanExecute for CarefulObservation {
    fn can_execute<C, M>(&self, state: &crate::CraftingState<C, M>) -> bool
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        state.buffs.specialist_actions.actions_available()
    }
}
