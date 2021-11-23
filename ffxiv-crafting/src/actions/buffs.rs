//! Contains the stage of an action where it affects the buff state, as well as actions whose primary purpose is to buff the player.

use crate::{buffs::BuffState, conditions::Condition, quality_map::QualityMap, CraftingState};

pub use concrete::*;

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
}

mod concrete {
    use crate::{
        actions::CanExecute,
        buffs::{quality::InnerQuietBaseStacks, Buff, DurationalBuff},
        BuffState, CraftingState,
    };
    use ffxiv_crafting_derive::*;

    use super::BuffAction;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, TimePassing)]
    #[derive(ActionLevel, RandomAction)]
    #[ffxiv_cp(cost = 18)]
    #[ffxiv_act_lvl(level = 11)]
    #[ffxiv_durability(cost = 0)]
    pub struct InnerQuiet;

    impl BuffAction for InnerQuiet {
        fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .quality
                .inner_quiet
                .activate(InnerQuietBaseStacks::InnerQuiet);
        }
    }

    impl CanExecute for InnerQuiet {
        fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            state.buffs.quality.inner_quiet.is_inactive()
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, TimePassing)]
    #[derive(CanExecute, ActionLevel, RandomAction)]
    #[ffxiv_cp(cost = 18)]
    #[ffxiv_act_lvl(level = 15)]
    #[ffxiv_durability(cost = 0)]
    pub struct Veneration;

    impl BuffAction for Veneration {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .progress
                .veneration
                .activate(state.condition.to_status_duration_modifier() as u8);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, TimePassing)]
    #[derive(CanExecute, ActionLevel, RandomAction)]
    #[ffxiv_cp(cost = 56)]
    #[ffxiv_act_lvl(level = 15)]
    #[ffxiv_durability(cost = 0)]
    pub struct WasteNot;

    impl BuffAction for WasteNot {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .durability
                .waste_not
                .activate(state.condition.to_status_duration_modifier() as u8);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 32)]
    #[ffxiv_act_lvl(level = 21)]
    #[ffxiv_durability(cost = 0)]
    pub struct GreatStrides;

    impl BuffAction for GreatStrides {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .quality
                .great_strides
                .activate(state.condition.to_status_duration_modifier() as u8);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 18)]
    #[ffxiv_act_lvl(level = 26)]
    #[ffxiv_durability(cost = 0)]
    pub struct Innovation;

    impl BuffAction for Innovation {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .quality
                .innovation
                .activate(state.condition.to_status_duration_modifier() as u8);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 30)]
    #[ffxiv_act_lvl(level = 37)]
    #[ffxiv_durability(cost = 0)]
    pub struct NameOfTheElements;

    impl BuffAction for NameOfTheElements {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .progress
                .veneration
                .activate(state.condition.to_status_duration_modifier() as u8);
        }
    }

    impl CanExecute for NameOfTheElements {
        fn can_execute<C, M>(&self, state: &CraftingState<C, M>) -> bool
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            state.buffs.progress.name_of_the_elements.can_activate()
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 1)]
    #[ffxiv_act_lvl(level = 42)]
    #[ffxiv_durability(cost = 0)]
    #[ffxiv_no_time_pass]
    pub struct FinalAppraisal;

    impl BuffAction for FinalAppraisal {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .progress
                .final_appraisal
                .activate(state.condition.to_status_duration_modifier() as u8);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 98)]
    #[ffxiv_act_lvl(level = 47)]
    #[ffxiv_durability(cost = 0)]
    pub struct WasteNot2;

    impl BuffAction for WasteNot2 {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .durability
                .waste_not
                .activate(4 + state.condition.to_status_duration_modifier() as u8);
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, BuffAction)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 88)]
    #[ffxiv_act_lvl(level = 7)]
    #[ffxiv_durability(bonus = 30)]
    pub struct MastersMend;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
    #[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
    #[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
    #[ffxiv_cp(cost = 96)]
    #[ffxiv_act_lvl(level = 65)]
    #[ffxiv_durability(cost = 0)]
    pub struct Manipulation;

    impl BuffAction for Manipulation {
        fn buff<C, M>(&self, state: &CraftingState<C, M>, so_far: &mut BuffState)
        where
            C: crate::conditions::Condition,
            M: crate::quality_map::QualityMap,
        {
            so_far
                .durability
                .manipulation
                .activate(state.condition.to_status_duration_modifier() as u8);
        }
    }
}
