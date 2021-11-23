use ffxiv_crafting_derive::*;

use super::{buffs::BuffAction, Action, CanExecute, CpCost, DurabilityFactor};
use crate::{quality_map::QualityMap, Condition, CraftingState};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
#[derive(ProgressAction, QualityAction, BuffAction)]
pub struct NullFailure<A: Action>(pub A);

impl<A: Action> DurabilityFactor for NullFailure<A> {
    const DURABILITY_USAGE: i8 = 0;

    fn durability<C>(&self, buffs: &crate::buffs::BuffState, condition: &C) -> i8
    where
        C: crate::conditions::Condition,
    {
        self.0.durability(buffs, condition)
    }
}

impl<A: Action> CpCost for NullFailure<A> {
    const CP_COST: i16 = 0;

    fn cp_cost<C, M>(&self, state: &CraftingState<C, M>) -> i16
    where
        C: Condition,
        M: QualityMap,
    {
        self.0.cp_cost(state)
    }
}

impl<A: Action> CanExecute for NullFailure<A> {
    fn can_execute<C, M>(&self, state: &crate::CraftingState<C, M>) -> bool
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        self.0.can_execute(state)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
#[derive(ProgressAction, QualityAction, CpCost, DurabilityFactor, CanExecute)]
#[ffxiv_cp(cost = 6)]
pub struct PatientFailure;

impl BuffAction for PatientFailure {
    fn buff<C, M>(&self, _: &CraftingState<C, M>, so_far: &mut crate::buffs::BuffState)
    where
        C: Condition,
        M: QualityMap,
    {
        so_far.quality.inner_quiet /= 2;
    }
}
