//! Contains definitions used with [`RandomAction`] for when an action fails its roll. These
//! don't implement [`RandomAction`] itself due to what that means not being entirely clear.
//!
//! [`RandomAction`]: crate::actions::RandomAction

use ffxiv_crafting_derive::*;

use super::{Action, ActionComponents, CanExecute, CpCost, DurabilityFactor};
use crate::{quality_map::QualityMap, Condition, CraftingState};

/// This is what happens when most [`RandomAction`]s fail - they just use their
/// CP and take off their durability but have no effect. This takes the action
/// and simply defers to its costs when being run through [`act`].
///
/// This is the [`FailAction`] chosen by default when deriving [`RandomAction`].
///
/// [`RandomAction`]: crate::actions::RandomAction
/// [`act`]: Action::act
/// [`FailAction`]: crate::actions::RandomAction::FailAction
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
#[derive(ProgressAction, QualityAction, BuffAction, TimePassing, Action)]
pub struct NullFailure<A: Action + ActionComponents>(pub A);

impl<A: Action + ActionComponents> DurabilityFactor for NullFailure<A> {
    const DURABILITY_USAGE: i8 = 0;

    fn durability<C>(&self, buffs: &crate::buffs::BuffState, condition: &C) -> i8
    where
        C: crate::conditions::Condition,
    {
        self.0.durability(buffs, condition)
    }
}

impl<A: Action + ActionComponents> CpCost for NullFailure<A> {
    const CP_COST: i16 = 0;

    fn cp_cost<C, M>(&self, state: &CraftingState<C, M>) -> i16
    where
        C: Condition,
        M: QualityMap,
    {
        self.0.cp_cost(state)
    }
}

impl<A: Action + ActionComponents> CanExecute for NullFailure<A> {
    fn can_execute<C, M>(&self, state: &crate::CraftingState<C, M>) -> bool
    where
        C: crate::conditions::Condition,
        M: crate::quality_map::QualityMap,
    {
        self.0.can_execute(state)
    }
}
