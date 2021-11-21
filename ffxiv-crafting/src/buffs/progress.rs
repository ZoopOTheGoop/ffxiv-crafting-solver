//! Contains buffs that modify the effects of actions on the `progress` attribute of the crafting state, such as [`Veneration`].

use derivative::Derivative;

/// A simple collection of all the progress buffs, for cleaner fields on simulation
/// structs.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct ProgressBuffs {
    pub brand_of_the_elements: BrandOfTheElements,
    pub veneration: Veneration,
    pub muscle_memory: MuscleMemory,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum BrandOfTheElements {
    #[derivative(Default)]
    Unused,
    Active(u8),
    Used,
}

// TODO: veneration is 4 steps, muscle memory is 5

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum Veneration {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum MuscleMemory {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}
