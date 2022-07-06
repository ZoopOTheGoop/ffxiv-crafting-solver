//! Defines the effects on quality that actions have, as well as collects actions whose primary purpose is increasing the quality property.

use crate::{
    actions::{buffs::BuffAction, CanExecute, CpCost},
    buffs::{combo::BasicTouchCombo, Buff, ConsumableBuff, DurationalBuff},
    conditions::Condition,
    quality_map::QualityMap,
    CraftingState,
};

/// An action's effect on the `quality` attribute. The
/// [`EFFICIENCY`](QualityAction::EFFICIENCY) is the base efficiency of the given
/// action, without any modifiers.
pub trait QualityAction {
    /// The base efficiency applied during the [`efficiency`] calculation, if 0 this will
    /// skip all calculations becuase the action doesn't affect this property.
    ///
    /// [`efficiency`]: QualityAction::efficiency
    const EFFICIENCY: u32 = 0;

    /// Calculates the efficiency of the current action on the crafting state. By default this is simply the efficiency bonus granted buffs,
    /// multiplied by the action's efficiency.
    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        if Self::EFFICIENCY == 0 {
            0.
        } else {
            (Self::EFFICIENCY * state.buffs.quality.efficiency_mod()) as f64 / 100.
        }
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

        ((quality * condition_mod * efficiency) / 100.) as u32
    }
}

use ffxiv_crafting_derive::*;

/// The most basic quality increasing action. Combos with [`StandardTouch`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 18)]
#[ffxiv_quality(efficiency = 100)]
#[ffxiv_act_lvl(level = 5)]
pub struct BasicTouch;

impl BuffAction for BasicTouch {
    fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: Condition,
        M: QualityMap,
    {
        so_far.combo.basic_touch.activate_in_place(0);
        so_far.quality.inner_quiet += 1;
    }

    fn deactivate_buff<C, M>(
        &self,
        _state: &CraftingState<C, M>,
        so_far: &mut crate::buffs::BuffState,
    ) where
        C: Condition,
        M: QualityMap,
    {
        if so_far.quality.great_strides.is_active() {
            so_far.quality.great_strides.deactivate_in_place();
        }
    }
}

/// A risky, costless quality increasing move. Will do nothing but consume
/// durability if it fails.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 0)]
#[ffxiv_quality(efficiency = 100)]
#[ffxiv_act_lvl(level = 9)]
#[ffxiv_rand_act(fail_rate = 40)]
#[ffxiv_buff_act(touch)]
pub struct HastyTouch;

/// An inefficient quality increasing action. This becomes efficient if combo'd
/// with [`BasicTouch`] as it becomes 25 bonus efficiency for the same cost.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_quality(efficiency = 125)]
#[ffxiv_act_lvl(level = 9)]
pub struct StandardTouch;

impl CpCost for StandardTouch {
    const CP_COST: i16 = 32;

    fn cp_cost<C, M>(&self, state: &CraftingState<C, M>) -> i16
    where
        C: Condition,
        M: QualityMap,
    {
        let cost = if matches!(state.buffs.combo.basic_touch, BasicTouchCombo::BasicTouch) {
            BasicTouch::CP_COST
        } else {
            Self::CP_COST
        };

        let condition_mod = state.condition.to_cp_usage_modifier() as u64 as f64 / 100.;

        // Todo: verify where floor/ceil might be
        (cost as f64 * condition_mod) as i16
    }
}

impl BuffAction for StandardTouch {
    fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: Condition,
        M: QualityMap,
    {
        so_far.quality.inner_quiet += 1;
        if matches!(so_far.combo.basic_touch, BasicTouchCombo::BasicTouch) {
            so_far.combo.basic_touch = BasicTouchCombo::StandardTouch;
        }
    }

    fn deactivate_buff<C, M>(
        &self,
        _state: &CraftingState<C, M>,
        so_far: &mut crate::buffs::BuffState,
    ) where
        C: Condition,
        M: QualityMap,
    {
        if so_far.quality.great_strides.is_active() {
            so_far.quality.great_strides.deactivate_in_place();
        }
    }
}

/// A powerful finishing action which consumes all stacks of [`InnerQuiet`] for a large payout,
/// 20% per stack on top of the 10% modifier to efficiency already granted by IQ.
///
/// [`InnerQuiet`]: crate::buffs::quality::InnerQuiet
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, DurabilityFactor, CpCost)]
#[derive(RandomAction, ActionLevel, TimePassing, Action)]
#[ffxiv_cp(cost = 24)]
#[ffxiv_act_lvl(level = 50)]
pub struct ByregotsBlessing;

impl CanExecute for ByregotsBlessing {
    fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
    where
        C: Condition,
        M: QualityMap,
    {
        state.buffs.quality.inner_quiet.is_active()
    }
}

impl QualityAction for ByregotsBlessing {
    const EFFICIENCY: u32 = 100;

    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        // This is technically stacks * 30 if you do the math but it's clearer both bonuses are being applied this way
        //
        // Further note: the patch notes note that efficiency bonus is limited to 300 but that seems natural.
        // 200 from the Byregot-specific mechanic, and 100 from base efficiency.
        let efficiency = Self::EFFICIENCY + (state.buffs.quality.inner_quiet.stacks() as u32 * 20);

        (efficiency * state.buffs.quality.efficiency_mod()) as f64 / 100.
    }
}

impl BuffAction for ByregotsBlessing {
    fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: Condition,
        M: QualityMap,
    {
        so_far.quality.inner_quiet.deactivate_in_place();
    }

    fn deactivate_buff<C, M>(
        &self,
        _state: &CraftingState<C, M>,
        so_far: &mut crate::buffs::BuffState,
    ) where
        C: Condition,
        M: QualityMap,
    {
        if so_far.quality.great_strides.is_active() {
            so_far.quality.great_strides.deactivate_in_place();
        }
    }
}

/// An efficient quality-increasing action that costs as much as [`BasicTouch`], and has
/// 25 more efficiency than [`StandardTouch`], as well as adding 2 stacks to [`InnerQuiet`]
/// instead of 1 -- but it can only be used if the [`Condition`] is [`Good`] or [`Excellent`], or
/// [`HeartAndSoul`] is active.
///
/// [`InnerQuiet`]: crate::buffs::quality::InnerQuiet
/// [`Good`]: crate::conditions::QARegularConditions::Good
/// [`Excellent`]: crate::conditions::QARegularConditions::Excellent
/// [`HeartAndSoul`]: crate::buffs::misc::HeartAndSoul
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_quality(efficiency = 150)]
#[ffxiv_act_lvl(level = 53)]
#[ffxiv_cp(cost = 18)]
#[ffxiv_can_exe(class = "good_excellent")]
pub struct PreciseTouch;

impl BuffAction for PreciseTouch {
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

        if so_far.quality.great_strides.is_active() {
            so_far.quality.great_strides.deactivate_in_place();
        }
    }

    fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: Condition,
        M: QualityMap,
    {
        so_far.quality.inner_quiet += 2;
    }
}

/// A slightly more expensive quality increasing action that uses half the normal durability, but otherwise
/// has the same efficiency as [`BasicTouch`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
#[derive(BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_quality(efficiency = 100)]
#[ffxiv_act_lvl(level = 66)]
#[ffxiv_cp(cost = 25)]
#[ffxiv_buff_act(touch)]
#[ffxiv_durability(cost = 5)]
pub struct PrudentTouch;

impl CanExecute for PrudentTouch {
    fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
    where
        C: Condition,
        M: QualityMap,
    {
        state.buffs.durability.waste_not.is_inactive()
    }
}

/// An action with the same cost as [`BasicTouch`], but 25 more efficiency than
/// [`StandardTouch`]. However, it has a 50% failure rate unless used immediately after
/// [`Observe`] making its expected value the same as [`StandardTouch`] in terms of
/// efficiency, but its expected durability and CP cost strictly worse.
///
/// [`Observe`]: crate::actions::misc::Observe
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_quality(efficiency = 150)]
#[ffxiv_act_lvl(level = 68)]
#[ffxiv_cp(cost = 18)]
#[ffxiv_buff_act(touch)]
#[ffxiv_rand_act(chance = 50, class = "combo_observe")]
pub struct FocusedTouch;

/// A starter action that can only be used on the first turn. It raises quality as much as a
/// [`BasicTouch`], for a little extra CP. However, it starts your [`InnerQuiet`] stacks off
/// at 3 instead of the usual 1.
///
/// [`InnerQuiet`]: crate::buffs::quality::InnerQuiet
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost, BuffAction)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_quality(efficiency = 100)]
#[ffxiv_act_lvl(level = 69)]
#[ffxiv_cp(cost = 6)]
#[ffxiv_can_exe(class = "first_step")]
#[ffxiv_buff_act(touch = 2)]
pub struct Reflect;

/// A very expensive action with twice as much efficiency as [`BasicTouch`], and gives two [`InnerQuiet`] stacks at once.
/// Its durability cost is the same as doing two [`BasicTouch`]es in a row, with its CP
/// cost being about 4 more than that, so it ends up being 50 efficiency and 4 CP inferior to the
/// [`BasicTouch`]+[`StandardTouch`] combo.
///
/// **However**, it makes much more efficient use of buff durations,
/// being able to fit in, for instance, 600 base efficiency per [`Innovation`] rather than the 500 the
/// combo would be able to in the same space, leading to getting 50 more efficiency out of that buff alone,
/// and its appeal only increases as you add in more buffs.
///
/// [`Innovation`]: crate::buffs::quality::Innovation
/// [`InnerQuiet`]: crate::buffs::quality::InnerQuiet
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_quality(efficiency = 200)]
#[ffxiv_act_lvl(level = 71)]
#[ffxiv_cp(cost = 40)]
#[ffxiv_buff_act(touch = 2)]
#[ffxiv_durability(cost = 20)]
pub struct PreparatoryTouch;

/// A special powerful action only usable on recipes 10 levels below the player. It instantly
/// maxes out quality, giving a guaranteed HQ or max collectability item (assuming the player can
/// finish its `progress`, but at a 10 level advantage that's extremely likely).
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, DurabilityFactor, CpCost)]
#[derive(BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_act_lvl(level = 80)]
#[ffxiv_cp(cost = 250)]
#[ffxiv_durability(cost = 0)]
pub struct TrainedEye;

impl QualityAction for TrainedEye {
    fn quality<C, M>(&self, state: &CraftingState<C, M>) -> u32
    where
        C: Condition,
        M: QualityMap,
    {
        state.problem_def.recipe.max_quality
    }
}

impl CanExecute for TrainedEye {
    fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
    where
        C: Condition,
        M: QualityMap,
    {
        state.first_step
            && !C::EXPERT
            && (state.problem_def.character.char_level as i8
                - state.problem_def.recipe.required_character_level as i8)
                >= 10
    }
}

/// A high-efficiency quality action. Combos off of [`StandardTouch`], if it combo'd off [`BasicTouch].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action, BuffAction)]
#[ffxiv_quality(efficiency = 150)]
#[ffxiv_act_lvl(level = 84)]
#[ffxiv_buff_act(touch)]
pub struct AdvancedTouch;

impl CpCost for AdvancedTouch {
    const CP_COST: i16 = 32;

    fn cp_cost<C, M>(&self, state: &CraftingState<C, M>) -> i16
    where
        C: Condition,
        M: QualityMap,
    {
        let cost = if matches!(
            state.buffs.combo.basic_touch,
            BasicTouchCombo::StandardTouch
        ) {
            BasicTouch::CP_COST
        } else {
            Self::CP_COST
        };

        let condition_mod = state.condition.to_cp_usage_modifier() as u64 as f64 / 100.;

        // Todo: verify where floor/ceil might be
        (cost as f64 * condition_mod) as i16
    }
}

/// A special action that's only as strong as [`BasicTouch`], but has no combo and is only usable
/// as 10 [`InnerQuiet`] stacks. The tradeoff is it costs no durability.
///
/// [`InnerQuiet`]: crate::buffs::quality::InnerQuiet
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, BuffAction)]
#[derive(ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 32)]
#[ffxiv_quality(efficiency = 100)]
#[ffxiv_act_lvl(level = 90)]
pub struct TrainedFinesse;

impl CanExecute for TrainedFinesse {
    fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
    where
        C: Condition,
        M: QualityMap,
    {
        state.buffs.quality.inner_quiet.stacks() == 10
    }
}
