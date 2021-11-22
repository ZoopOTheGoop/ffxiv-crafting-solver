//! Submodules of this module define actions that act on a [`CraftingState`](crate::CraftingState) and yield the next one.

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

pub mod buffs;
pub mod progress;
pub mod quality;

pub trait Action: Sized {
    fn change_state<C, M>(self, state: &mut CraftingState<C, M>)
    where
        C: Condition,
        M: QualityMap;
}
