//! Defines the effects on progress that actions have, as well as collects actions whose primary purpose is increasing the progress property.

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

pub use self::concrete::*;

/// An action's effect on the `progress` attribute. The
/// [`EFFICIENCY`](ProgressAction::EFFICIENCY) is the base efficiency of the given
/// action, without any modifiers.
pub trait ProgressAction {
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
        if Self::EFFICIENCY == 0 {
            return 0;
        }

        let progress = state.base_progress();
        let condition_mod = state.condition.to_progress_modifier() as u64 as f64 / 100.;
        let efficiency = self.efficiency(state);

        (progress.floor() * condition_mod * efficiency) as u32
    }
}

mod concrete {
    use ffxiv_crafting_derive::*;

    use crate::{
        actions::{buffs::BuffAction, progress::ProgressAction, DurabilityFactor},
        buffs::{Buff, BuffState, DurationalBuff},
        conditions::Condition,
        quality_map::QualityMap,
        CraftingState,
    };

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 0)]
    #[ffxiv_progress(efficiency = 100)]
    #[ffxiv_act_lvl(level = 1)]
    pub struct BasicSynthesis;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 0)]
    #[ffxiv_progress(efficiency = 500)]
    #[ffxiv_act_lvl(level = 9)]
    #[ffxiv_rand_act(fail_rate = 50)]
    pub struct RapidSynthesis;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 6)]
    #[ffxiv_act_lvl(level = 37)]
    pub struct BrandOfTheElements;

    impl ProgressAction for BrandOfTheElements {
        const EFFICIENCY: u16 = 100;

        fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
        where
            C: Condition,
            M: QualityMap,
        {
            let efficiency_mod = 100. + state.buffs.progress.efficiency_mod() as f64 / 100.;

            let efficiency = efficiency_mod * Self::EFFICIENCY as f64;

            if state.buffs.progress.name_of_the_elements.is_active() {
                efficiency
                    + 2. * ((1.
                        - state.curr_progress as f64
                            / state.problem_def.recipe.max_progress as f64)
                        * 100.)
                        .ceil()
            } else {
                efficiency
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 6)]
    #[ffxiv_progress(efficiency = 300)]
    #[ffxiv_act_lvl(level = 54)]
    #[ffxiv_can_exe(class = "first_step")]
    pub struct MuscleMemory;

    impl BuffAction for MuscleMemory {
        fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: Condition,
            M: QualityMap,
        {
            // Don't bother with condition modifier -- first state is always normal
            so_far.progress.muscle_memory.activate(0);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 7)]
    #[ffxiv_progress(efficiency = 150)]
    #[ffxiv_act_lvl(level = 62)]
    pub struct CarefulSynthesis;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 5)]
    #[ffxiv_progress(efficiency = 200)]
    #[ffxiv_act_lvl(level = 67)]
    #[ffxiv_rand_act(fail_rate = 50, class = "combo_observe")]
    pub struct FocusedSynthesis;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 0)]
    #[ffxiv_act_lvl(level = 72)]
    #[ffxiv_durability(cost = 20)]
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

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 6)]
    #[ffxiv_progress(efficiency = 400)]
    #[ffxiv_act_lvl(level = 78)]
    #[ffxiv_can_exe(class = "good_excellent")]
    pub struct IntensiveSynthesis;
}
