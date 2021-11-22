use crate::{buffs::BuffState, conditions::Condition, quality_map::QualityMap, CraftingState};

pub trait BuffAction {
    fn buff<C, M>(&self, state: &CraftingState<C, M>) -> BuffState
    where
        C: Condition,
        M: QualityMap;
}
