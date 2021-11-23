use ffxiv_crafting_derive::*;

use crate::buffs::DurationalBuff;

use super::{buffs::BuffAction, CanExecute};

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
#[ffxiv_cp(cost = 7)]
#[ffxiv_act_lvl(level = 13)]
#[ffxiv_durability(cost = 0)]
pub struct Observe;

impl BuffAction for Observe {
    fn buff<C, M>(&self, _: &crate::CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        so_far.combo.observation.activate(0);
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, BuffAction)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
#[ffxiv_cp(bonus = 20)]
#[ffxiv_act_lvl(level = 13)]
#[ffxiv_can_exe(class = "good_excellent")]
pub struct TricksOfTheTrade;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, BuffAction)]
#[derive(CanExecute, ActionLevel, RandomAction, TimePassing)]
#[ffxiv_cp(cost = 32)]
#[ffxiv_act_lvl(level = 76)]
#[ffxiv_quality(efficiency = 100)]
#[ffxiv_progress(efficiency = 100)]
#[ffxiv_buff_act(class = "touch")]
pub struct DelicateSynthesis;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor)]
#[derive(ActionLevel, RandomAction, TimePassing)]
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
