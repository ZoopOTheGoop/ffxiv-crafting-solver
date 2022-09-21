//! Contains the stage of an action where it affects the buff state, as well as actions whose primary purpose is to buff the player.

use crate::{
    actions::CanExecute,
    buffs::{BuffState, DurationalBuff},
    conditions::Condition,
    quality_map::QualityMap,
    CraftingState,
};
use ffxiv_crafting_derive::*;

/// Defines the buffs (if any) and action applies to the current state.
pub trait BuffAction {
    /// Adds buffs to the current state. This takes in `so_far` which holds
    /// any buffs that may already have been applied (e.g. the [`quality`] stage
    /// applies [`InnerQuiet`] early for "touch" actions).
    ///
    /// [`quality`]: crate::actions::quality
    /// [`InnerQuiet`]: crate::buffs::quality::InnerQuiet
    #[allow(unused_variables)]
    fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
    where
        C: Condition,
        M: QualityMap,
    {
    }

    // TODO: Update `touch` in the derive crate to put other deactivations in here, currently only manipulation is
    // because it was added after I realized it mattered (and it *only* matters for Manpulation), but it'd be good
    // to have other deactivations here for consistency.

    /// Deactivates and consumes buffs in the current state. The difference between this and
    /// [`buff`] is simply timing - this is done before [`Manipulation`] repairs the item, because
    /// [`Manipulation`] does not work on the same turn it applies.
    ///
    /// [`buff`]: BuffAction::buff
    #[allow(unused_variables)]
    fn deactivate_buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
    where
        C: Condition,
        M: QualityMap,
    {
    }
}

/// Activates the [`Veneration`] buff, adding 0.5 onto the efficiency multiplier for
/// [synthesis] (progress increasing) actions for 4 steps.
///
/// [`Veneration`]: crate::buffs::progress::Veneration
/// [synthesis]: crate::actions::progress
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, TimePassing)]
#[derive(CanExecute, ActionLevel, RandomAction, BuffAction, Action)]
#[ffxiv_cp(cost = 18)]
#[ffxiv_act_lvl(level = 15)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_buff_act(activate = "progress.veneration")]
pub struct Veneration;

/// Activates the [`WasteNot`] buff for 4 steps, reducing durability cost by half for all
/// actions, but disabling [`PrudentTouch`].
///
/// [`WasteNot`]: crate::buffs::durability::WasteNot
/// [`PrudentTouch`]: crate::actions::quality::PrudentTouch
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, TimePassing)]
#[derive(CanExecute, ActionLevel, RandomAction, BuffAction, Action)]
#[ffxiv_cp(cost = 56)]
#[ffxiv_act_lvl(level = 15)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_buff_act(activate = "durability.waste_not")]
pub struct WasteNot;

/// Activates the [`GreatStrides`] buff, adding 1.0 onto the efficiency multiplier for
/// [touch] (quality increasing) actions for up to 3 steps, but being consumed immediately
/// upon use.
///
/// [`GreatStrides`]: crate::buffs::quality::GreatStrides
/// [touch]: crate::actions::quality
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, BuffAction, Action)]
#[ffxiv_cp(cost = 32)]
#[ffxiv_act_lvl(level = 21)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_buff_act(activate = "quality.great_strides")]
pub struct GreatStrides;

/// Activates the [`Innovation`] buff, adding 0.5 onto the efficiency multiplier for
/// [touch] (quality increasing) actions for 4 steps.
///
/// [`GreatStrides`]: crate::buffs::quality::Innovation
/// [touch]: crate::actions::quality
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, BuffAction, Action)]
#[ffxiv_cp(cost = 18)]
#[ffxiv_act_lvl(level = 26)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_buff_act(activate = "quality.innovation")]
pub struct Innovation;

/// Activates the [`FinalAppraisal`] buff for 5 steps, "catching" the first
/// [synthesis] (progress increasing) action that would cause the recipe to complete
/// and leaving the recipe with only 1 progress remaining until completing.
///
/// This is a special action that "freezes time" so to speak. When using it buffs don't
/// decay nor do things like [`Manipulation`] tick (though combos do get interrupted), but
/// the condition does cycle. This means it can be used on the first step "before"
/// [`MuscleMemory`] to instantly leave a recipe away from one completion.
///
/// [`FinalAppraisal`]: crate::buffs::progress::FinalAppraisal
/// [synthesis]: crate::actions::progress
/// [`MuscleMemory`]: crate::actions::progress::MuscleMemory
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, BuffAction, Action)]
#[ffxiv_cp(cost = 1)]
#[ffxiv_act_lvl(level = 42)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_no_time_pass]
#[ffxiv_buff_act(activate = "progress.final_appraisal")]
pub struct FinalAppraisal;

/// Activates the [`WasteNot`] buff's [`WasteNot2`] variant for 8 steps,
/// reducing durability cost by half for all actions, but disabling [`PrudentTouch`].
///
/// [`WasteNot`]: crate::buffs::durability::WasteNot
/// [`WasteNot2`]: crate::buffs::durability::WasteNot::WasteNot2
/// [`PrudentTouch`]: crate::actions::quality::PrudentTouch
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 98)]
#[ffxiv_act_lvl(level = 47)]
#[ffxiv_durability(cost = 0)]
pub struct WasteNot2;

// Need a manual impl because of the 4 bonus. Custom base durations are not worth the headache
impl BuffAction for WasteNot2 {
    fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        so_far
            .durability
            .waste_not
            .activate_in_place(4 + state.condition.to_status_duration_modifier() as u8);
    }
}

/// Activates the [`Manipulation`] buff for 8 steps, causing 5 durability to be restored at the
/// end of every subsequent turn. However, this repair is done *after* completion is checked,
/// so if the item breaks this won't save it.
///
/// [`Manipulation`]: crate::buffs::durability::Manipulation
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, BuffAction, Action)]
#[ffxiv_cp(cost = 96)]
#[ffxiv_act_lvl(level = 65)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_buff_act(
    activate = "durability.manipulation",
    consume = "durability.manipulation"
)]
pub struct Manipulation;

/// A strong specialist Endwalker action that is associated with the [`HeartAndSoul`] buff. It
/// allows actions such as [`TricksOfTheTrade`] to be activated even when the condition is not Good or Excellent.
///
/// [`HeartAndSoul`]: crate::buffs::misc::HeartAndSoul
/// [`TricksOfTheTrade`]: crate::actions::misc::TricksOfTheTrade
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 0)]
#[ffxiv_act_lvl(level = 86)]
#[ffxiv_durability(cost = 0)]
#[ffxiv_no_time_pass]
pub struct HeartAndSoul;

impl BuffAction for HeartAndSoul {
    fn buff<C, M>(&self, _: &crate::CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        so_far.specialist_actions -= 1;
        so_far.heart_and_soul.activate_in_place();
    }
}

impl CanExecute for HeartAndSoul {
    fn can_execute<C, M>(&self, state: &crate::CraftingState<C, M>) -> bool
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        state.buffs.specialist_actions.actions_available()
    }
}
