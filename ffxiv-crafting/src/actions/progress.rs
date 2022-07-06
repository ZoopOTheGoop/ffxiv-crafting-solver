//! Defines the effects on progress that actions have, as well as collects actions whose primary purpose is increasing the progress property.

use crate::{
    actions::{buffs::BuffAction, CanExecute, DurabilityFactor},
    buffs::{Buff, ConsumableBuff},
    conditions::Condition,
    quality_map::QualityMap,
    CraftingState,
};

/// An action's effect on the `progress` attribute. The
/// [`EFFICIENCY`](ProgressAction::EFFICIENCY) is the base efficiency of the given
/// action, without any modifiers.
pub trait ProgressAction {
    /// The base efficiency applied during the [`efficiency`] calculation, if 0 this will
    /// skip all calculations becuase the action doesn't affect this property.
    ///
    /// [`efficiency`]: ProgressAction::efficiency
    const EFFICIENCY: u16 = 0;

    /// Calculates the base efficiency for this action. While theoretically this is
    /// covered by the [`EFFICIENCY`] constant, a surprising number of synthesis actions
    /// modify this based on level or context, but not the overall calculation.
    ///
    /// [`EFFICIENCY`]: ProgressAction::EFFICIENCY
    #[allow(unused_variables)]
    fn base_efficiency<C, M>(&self, state: &CraftingState<C, M>) -> u16
    where
        C: Condition,
        M: QualityMap,
    {
        Self::EFFICIENCY
    }

    /// Calculates the efficiency of the current action on the crafting state.
    /// By default this is simply the efficiency bonus granted buffs,
    /// multiplied by the action's efficiency.
    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        if Self::EFFICIENCY == 0 {
            return 0.;
        }

        let efficiency = self.base_efficiency(state) + state.buffs.progress.bonus_efficiency();
        let efficiency_mod = (100. + state.buffs.progress.efficiency_mod() as f64) / 100.;

        (efficiency_mod * efficiency as f64) / 100.
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
        if Self::EFFICIENCY == 0 {
            return 0;
        }

        let progress = state.base_progress();
        let condition_mod = state.condition.to_progress_modifier() as u64 as f64 / 100.;
        let efficiency = self.efficiency(state);

        (progress * condition_mod * efficiency) as u32
    }
}

use ffxiv_crafting_derive::*;

use super::{failure::NullFailure, RandomAction};

/// The most basic progress-increasing ability. It has 120 efficiency (after level 31) and no cost,
/// other than the standard 10 durability taken off.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 0)]
#[ffxiv_act_lvl(level = 1)]
#[ffxiv_buff_act(synthesis)]
pub struct BasicSynthesis;

impl ProgressAction for BasicSynthesis {
    const EFFICIENCY: u16 = 100;

    fn base_efficiency<C, M>(&self, state: &CraftingState<C, M>) -> u16
    where
        C: Condition,
        M: QualityMap,
    {
        if state.problem_def.character.char_level >= 31 {
            120
        } else {
            Self::EFFICIENCY
        }
    }
}

/// A risky, CP-free synthesis move that's extremely efficient, with a bit over 4x the amount
/// of efficiency as [`BasicSynthesis`]. However, it fails 50% of the time, simply damaging the item.
///
/// That said, its expected value is still twice as efficient as [`BasicSynthesis`], before taking into account
/// the power of consistently leveraging buffs.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 0)]
#[ffxiv_act_lvl(level = 9)]
#[ffxiv_rand_act(fail_rate = 50)]
#[ffxiv_buff_act(synthesis)]
pub struct RapidSynthesis;

impl ProgressAction for RapidSynthesis {
    const EFFICIENCY: u16 = 250;

    fn base_efficiency<C, M>(&self, state: &CraftingState<C, M>) -> u16
    where
        C: Condition,
        M: QualityMap,
    {
        if state.problem_def.character.char_level >= 63 {
            500
        } else {
            Self::EFFICIENCY
        }
    }
}

/// A powerful action that can only be used on the first step. It has 2.5x the efficiency of
/// [`BasicSynthesis`], and adds a buff that will cause the next progress-increasing action
/// to gain 100 bonus efficiency
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, BuffAction)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 6)]
#[ffxiv_progress(efficiency = 300)]
#[ffxiv_act_lvl(level = 54)]
#[ffxiv_can_exe(class = "first_step")]
#[ffxiv_buff_act(activate = "progress.muscle_memory")]
pub struct MuscleMemory;

/// An action that's only minorly more powerful than [`BasicSynthesis`], but
/// also doesn't cost very much.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 7)]
#[ffxiv_act_lvl(level = 62)]
#[ffxiv_buff_act(synthesis)]
pub struct CarefulSynthesis;

impl ProgressAction for CarefulSynthesis {
    const EFFICIENCY: u16 = 150;

    fn base_efficiency<C, M>(&self, state: &CraftingState<C, M>) -> u16
    where
        C: Condition,
        M: QualityMap,
    {
        if state.problem_def.character.char_level >= 82 {
            180
        } else {
            Self::EFFICIENCY
        }
    }
}

/// An cheap action with 200 efficiency, about ~1.7x that of [`BasicSynthesis`] that costs 5 CP,
/// (so 50 extra efficiency for 2 less CP than [`CarefulSynthesis`]).
///
///  However, it has a 50% failure rate unless used immediately after
/// [`Observe`] making its expected value relatively bad unless you had to use
/// observe anyway.
///
/// [`Observe`]: crate::actions::misc::Observe
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, TimePassing, Action)]
#[ffxiv_cp(cost = 5)]
#[ffxiv_progress(efficiency = 200)]
#[ffxiv_act_lvl(level = 67)]
#[ffxiv_buff_act(synthesis)]
pub struct FocusedSynthesis;

impl RandomAction for FocusedSynthesis {
    const FAIL_RATE: u8 = 50;

    type FailAction = NullFailure<Self>;

    fn fail_rate<C: Condition, M: QualityMap>(&self, state: &CraftingState<C, M>) -> u8 {
        if !state.buffs.combo.observation.is_active() {
            Self::FAIL_RATE
        } else {
            0
        }
    }

    fn fail_action(&self) -> Self::FailAction {
        NullFailure(*self)
    }
}

/// An action with as much efficiency as [`MuscleMemory`] that can be used at any time.
/// However, it costs twice the normal amount of durability and 18 CP, and only uses
/// half its efficiency if the item has less durability left than it would use (after condition
/// and buffs are taken into account), making it a very expensive [`CarefulSynthesis`] in that case.
///
/// Similar to [`PreparatoryTouch`], this is mainly useful for leveraging buffs such as cashing out
/// [`MuscleMemory`]'s buff, or making use of [`Veneration`].
///
/// [`PreparatoryTouch`]: crate::actions::quality::PreparatoryTouch
/// [`Veneration`]: crate::buffs::progress::Veneration
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 18)]
#[ffxiv_act_lvl(level = 72)]
#[ffxiv_durability(cost = 20)]
#[ffxiv_buff_act(synthesis)]
pub struct Groundwork;

impl ProgressAction for Groundwork {
    const EFFICIENCY: u16 = 300;

    fn base_efficiency<C, M>(&self, state: &CraftingState<C, M>) -> u16
    where
        C: Condition,
        M: QualityMap,
    {
        let efficiency = if state.problem_def.character.char_level >= 86 {
            360
        } else {
            Self::EFFICIENCY
        };

        let durability = self.durability(&state.buffs, &state.condition);
        if state.curr_durability + durability < 0 {
            efficiency / 2
        } else {
            efficiency
        }
    }
}

/// A cheap, high efficiency (3 1/3 times more efficiency than [`BasicSynthesis`]) action
/// that can only be used during [`Good`] or [`Excellent`] [`Condition`]s.
///
/// [`Good`]: crate::conditions::QARegularConditions::Good
/// [`Excellent`]: crate::conditions::QARegularConditions::Excellent
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 6)]
#[ffxiv_progress(efficiency = 400)]
#[ffxiv_act_lvl(level = 78)]
#[ffxiv_can_exe(class = "good_excellent")]
pub struct IntensiveSynthesis;

impl BuffAction for IntensiveSynthesis {
    fn deactivate_buff<C, M>(
        &self,
        state: &crate::CraftingState<C, M>,
        so_far: &mut crate::buffs::BuffState,
    ) where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        if !(state.condition.is_excellent() || state.condition.is_good())
            && so_far.heart_and_soul.is_active()
        {
            so_far.heart_and_soul.deactivate_in_place();
        }

        if so_far.progress.muscle_memory.is_active() {
            so_far.progress.muscle_memory.deactivate_in_place();
        }
    }
}

/// A slightly more expensive progress increasing action that uses half the normal durability, but otherwise
/// has the same efficiency as [`CarefulSynthesis`] (after its trait buff at level 82).
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
#[derive(BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_progress(efficiency = 180)]
#[ffxiv_act_lvl(level = 88)]
#[ffxiv_cp(cost = 18)]
#[ffxiv_buff_act(synthesis)]
#[ffxiv_durability(cost = 5)]
pub struct PrudentSynthesis;

impl CanExecute for PrudentSynthesis {
    fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
    where
        C: Condition,
        M: QualityMap,
    {
        state.buffs.durability.waste_not.is_inactive()
    }
}
