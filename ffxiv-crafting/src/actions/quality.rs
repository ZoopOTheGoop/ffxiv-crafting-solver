//! Defines the effects on quality that actions have, as well as collects actions whose primary purpose is increasing the quality property.

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

pub use self::concrete::*;

/// An action's effect on the `quality` attribute. The
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

mod concrete {
    use ffxiv_crafting_derive::*;

    use crate::{
        actions::{buffs::BuffAction, failure::PatientFailure, CanExecute, CpCost, RandomAction},
        buffs::{quality::InnerQuietBaseStacks, Buff, ConsumableBuff},
        conditions::Condition,
        quality_map::QualityMap,
        CraftingState,
    };

    use super::QualityAction;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 18)]
    #[ffxiv_quality(efficiency = 100)]
    #[ffxiv_act_lvl(level = 5)]
    #[ffxiv_buff_act(class = "touch")]
    pub struct BasicTouch;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 0)]
    #[ffxiv_quality(efficiency = 100)]
    #[ffxiv_act_lvl(level = 9)]
    #[ffxiv_rand_act(fail_rate = 40)]
    #[ffxiv_buff_act(class = "touch")]
    pub struct HastyTouch;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, DurabilityFactor)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_quality(efficiency = 125)]
    #[ffxiv_act_lvl(level = 9)]
    #[ffxiv_buff_act(class = "touch")]
    pub struct StandardTouch;

    impl CpCost for StandardTouch {
        const CP_COST: i16 = 32;

        fn cp_cost<C, M>(&self, state: &CraftingState<C, M>) -> i16
        where
            C: Condition,
            M: QualityMap,
        {
            let cost = if state.last_state_was_basic_touch {
                BasicTouch::CP_COST
            } else {
                Self::CP_COST
            };

            let condition_mod = state.condition.to_cp_usage_modifier() as u64 as f64 / 100.;

            // Todo: verify where floor/ceil might be
            (cost as f64 * condition_mod) as i16
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(
        ProgressAction,
        DurabilityFactor,
        CpCost,
        RandomAction,
        ActionLevel,
        TimePassing
    )]
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
        const EFFICIENCY: u16 = 100;

        fn efficiency<C, M>(&self, state: &CraftingState<C, M>) -> f64
        where
            C: Condition,
            M: QualityMap,
        {
            let efficiency_mod = 100. + state.buffs.quality.efficiency_mod() as f64 / 100.;
            let efficiency = 100. + (state.buffs.quality.inner_quiet.stacks() - 1) as f64 * 20.;

            efficiency_mod * efficiency
        }
    }

    impl BuffAction for ByregotsBlessing {
        fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
        where
            C: Condition,
            M: QualityMap,
        {
            so_far.quality.inner_quiet.deactivate();
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_quality(efficiency = 150)]
    #[ffxiv_act_lvl(level = 53)]
    #[ffxiv_cp(cost = 18)]
    #[ffxiv_buff_act(class = "touch", amount = 2)]
    #[ffxiv_can_exe(class = "good_excellent")]
    pub struct PreciseTouch;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost, TimePassing)]
    #[derive(CanExecute, ActionLevel)]
    #[ffxiv_quality(efficiency = 150)]
    #[ffxiv_act_lvl(level = 53)]
    #[ffxiv_cp(cost = 6)]
    #[ffxiv_can_exe(class = "good_excellent")]
    pub struct PatientTouch;

    impl RandomAction for PatientTouch {
        const FAIL_RATE: u8 = 50;

        type FailAction = PatientFailure;

        fn fail_action(&self) -> Self::FailAction {
            PatientFailure
        }
    }

    impl BuffAction for PatientTouch {
        fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
        where
            C: Condition,
            M: QualityMap,
        {
            so_far.quality.inner_quiet *= 2;
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
    #[derive(BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_quality(efficiency = 100)]
    #[ffxiv_act_lvl(level = 66)]
    #[ffxiv_cp(cost = 25)]
    #[ffxiv_buff_act(class = "touch")]
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

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_quality(efficiency = 150)]
    #[ffxiv_act_lvl(level = 68)]
    #[ffxiv_cp(cost = 18)]
    #[ffxiv_buff_act(class = "touch")]
    #[ffxiv_rand_act(chance = 50, class = "combo_observe")]
    pub struct FocusedTouch;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_quality(efficiency = 100)]
    #[ffxiv_act_lvl(level = 69)]
    #[ffxiv_cp(cost = 24)]
    #[ffxiv_can_exe(class = "first_step")]
    pub struct Reflect;

    impl BuffAction for Reflect {
        fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
        where
            C: Condition,
            M: QualityMap,
        {
            so_far
                .quality
                .inner_quiet
                .activate(InnerQuietBaseStacks::Reflection);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, DurabilityFactor, CpCost)]
    #[derive(CanExecute, BuffAction, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_quality(efficiency = 200)]
    #[ffxiv_act_lvl(level = 71)]
    #[ffxiv_cp(cost = 40)]
    #[ffxiv_buff_act(class = "touch", amount = 2)]
    #[ffxiv_durability(cost = 20)]
    #[ffxiv_can_exe(class = "good_excellent")]
    pub struct PreparatoryTouch;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, DurabilityFactor, CpCost)]
    #[derive(BuffAction, ActionLevel, RandomAction, TimePassing)]
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
            state.recipe.recipe.max_quality
        }
    }

    impl CanExecute for TrainedEye {
        fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
        where
            C: Condition,
            M: QualityMap,
        {
            state.first_step
                && (state.recipe.character.char_level as i8
                    - state.recipe.recipe.recipe_level.to_player_facing_level() as i8)
                    >= 10
        }
    }
}
