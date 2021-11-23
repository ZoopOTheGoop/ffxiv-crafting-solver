use ffxiv_crafting_derive::*;

use super::{Action, CanExecute, CpCost, DurabilityFactor};

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

    fn cp_cost<C>(&self, condition: &C) -> i16
    where
        C: crate::conditions::Condition,
    {
        self.0.cp_cost(condition)
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
