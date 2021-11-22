use crate::{buffs::BuffState, conditions::Condition, quality_map::QualityMap, CraftingState};

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
