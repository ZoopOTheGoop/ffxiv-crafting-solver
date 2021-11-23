use ffxiv_crafting_derive::*;

use crate::buffs::DurationalBuff;

use super::buffs::BuffAction;

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
