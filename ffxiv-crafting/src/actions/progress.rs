//! Defines the effects on progress that actions have, as well as collects actions whose primary purpose is increasing the progress property.

use crate::{
    actions::{CanExecute, DurabilityFactor},
    buffs::Buff,
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

        let efficiency = Self::EFFICIENCY + state.buffs.progress.bonus_efficiency();
        let efficiency_mod = (100. + state.buffs.progress.efficiency_mod() as f64) / 100.;

        efficiency_mod * efficiency as f64
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

        (progress.floor() * condition_mod * efficiency) as u32
    }
}

use ffxiv_crafting_derive::*;

/// The most basic progress-increasing ability. It has 120 efficiency and no cost,
/// other than the standard 10 durability taken off.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 0)]
#[ffxiv_progress(efficiency = 120)]
#[ffxiv_act_lvl(level = 1)]
#[ffxiv_buff_act(synthesis)]
pub struct BasicSynthesis;

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

    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        let efficiency = if state.problem_def.character.char_level >= 63 {
            500
        } else {
            Self::EFFICIENCY
        };

        let efficiency = efficiency + state.buffs.progress.bonus_efficiency();
        let efficiency_mod = (100. + state.buffs.progress.efficiency_mod() as f64) / 100.;

        efficiency_mod * efficiency as f64
    }
}

/// A complex progress action that appears to be less powerful than [`BasicSynthesis`] while
/// also cosing 6 CP. However, once per craft you may use the [`NameOfTheElements`] action to activate
/// its associated [buff][namebuff], and under said buff its efficiency increases to up to 200, attenuated
/// by the progress remaining in the item (lower = more powerful).
///
/// [namebuff]: crate::buffs::progress::NameOfTheElements
/// [`NameOfTheElements`]: crate::actions::buffs::NameOfTheElements
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 6)]
#[ffxiv_act_lvl(level = 37)]
#[ffxiv_buff_act(synthesis)]
pub struct BrandOfTheElements;

impl ProgressAction for BrandOfTheElements {
    const EFFICIENCY: u16 = 100;

    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        let efficiency = Self::EFFICIENCY + state.buffs.progress.bonus_efficiency();
        let efficiency_mod = 100. + state.buffs.progress.efficiency_mod() as f64 / 100.;

        let efficiency = efficiency_mod * efficiency as f64;

        if state.buffs.progress.name_of_the_elements.is_active() {
            efficiency
                + 2. * ((1.
                    - state.curr_progress as f64 / state.problem_def.recipe.max_progress as f64)
                    * 100.)
                    .ceil()
        } else {
            efficiency
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
#[ffxiv_buff_act(synthesis, activate = "progress.muscle_memory")]
pub struct MuscleMemory;

/// An action that's only minorly more powerful than [`BasicSynthesis`], but
/// also doesn't cost very much.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 7)]
#[ffxiv_progress(efficiency = 150)]
#[ffxiv_act_lvl(level = 62)]
#[ffxiv_buff_act(synthesis)]
pub struct CarefulSynthesis;

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
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 5)]
#[ffxiv_progress(efficiency = 200)]
#[ffxiv_act_lvl(level = 67)]
#[ffxiv_rand_act(fail_rate = 50, class = "combo_observe")]
#[ffxiv_buff_act(synthesis)]
pub struct FocusedSynthesis;

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

    fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
    where
        C: Condition,
        M: QualityMap,
    {
        let durability = self.durability(&state.buffs, &state.condition);
        let efficiency = if durability < state.curr_durability {
            Self::EFFICIENCY / 2
        } else {
            Self::EFFICIENCY
        };

        let efficiency_mod = 100. + state.buffs.progress.efficiency_mod() as f64 / 100.;

        efficiency_mod * efficiency as f64
    }
}

/// A cheap, high efficiency (3 1/3 times more efficiency than [`BasicSynthesis`]) action
/// that can only be used during [`Good`] or [`Excellent`] [`Condition`]s.
///
/// [`Good`]: crate::conditions::QARegularConditions::Good
/// [`Excellent`]: crate::conditions::QARegularConditions::Excellent
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_cp(cost = 6)]
#[ffxiv_progress(efficiency = 400)]
#[ffxiv_act_lvl(level = 78)]
#[ffxiv_can_exe(class = "good_excellent")]
#[ffxiv_buff_act(synthesis)]
pub struct IntensiveSynthesis;

// TODO: Look up CP cost for Prudent Synthesis, not in patch notes or in any wikis yet

/// A slightly more expensive progress increasing action that uses half the normal durability, but otherwise
/// has the same efficiency as [`CarefulSynthesis`] (after its trait buff at level 82).
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
#[derive(BuffAction, ActionLevel, RandomAction, TimePassing, Action)]
#[ffxiv_quality(efficiency = 180)]
#[ffxiv_act_lvl(level = 88)]
#[ffxiv_cp(cost = 0)]
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
