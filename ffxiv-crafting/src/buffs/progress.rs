//! Contains buffs that modify the effects of actions on the `progress` attribute of the crafting state, such as [`Veneration`].

use std::ops::{Sub, SubAssign};

use derivative::Derivative;

use super::{Buff, DurationalBuff};

/// A simple collection of all the progress buffs, for cleaner fields on simulation
/// structs.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct ProgressBuffs {
    pub brand_of_the_elements: NameOfTheElements,
    pub veneration: Veneration,
    pub muscle_memory: MuscleMemory,
}

impl ProgressBuffs {
    pub fn decay(&mut self) {
        self.brand_of_the_elements.decay_in_place();
        self.veneration.decay_in_place();
        self.muscle_memory.decay_in_place();
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum NameOfTheElements {
    #[derivative(Default)]
    Unused,
    Active(u8),
    Used,
}

impl Buff for NameOfTheElements {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for NameOfTheElements {
    const BASE_DURATION: u8 = 3;

    fn activate(self, bonus: u8) -> Self {
        match self {
            Self::Unused => Self::Active(Self::BASE_DURATION + bonus),
            Self::Active(_) | Self::Used => {
                panic!("Cannot activate Name of the Elements twice, check logic.")
            }
        }
    }
}

impl Sub<u8> for NameOfTheElements {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(val) => Self::Active(val - rhs),
            Self::Unused => Self::Unused,
            Self::Used => Self::Used,
        }
    }
}

impl SubAssign<u8> for NameOfTheElements {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

// TODO: veneration is 4 steps, muscle memory is 5

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum Veneration {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

impl Buff for Veneration {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for Veneration {
    const BASE_DURATION: u8 = 3;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl Sub<u8> for Veneration {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(val) => Self::Active(val - rhs),
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for Veneration {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum MuscleMemory {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

impl Buff for MuscleMemory {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for MuscleMemory {
    const BASE_DURATION: u8 = 3;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl Sub<u8> for MuscleMemory {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(val) => Self::Active(val - rhs),
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for MuscleMemory {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}
