//! Fake "buffs" that are really triggers for combo actions. [`MuscleMemory`] remains grouped with the [`progress`]
//! buffs because it's more appropriate there.
//!
//! [`MuscleMemory`]: crate::buffs::progress::MuscleMemory
//! [`progress`]: crate::buffs::progress

use std::ops::{Sub, SubAssign};

use derivative::Derivative;

use super::{Buff, DurationalBuff};

/// A collection of miscellaneous combo triggers that don't fit elsewhere.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct ComboTriggers {
    pub basic_touch: BasicTouchCombo,
    pub observation: ObservationCombo,
}

impl ComboTriggers {
    pub fn decay(&mut self) {
        self.basic_touch.decay_in_place();
        self.observation.decay_in_place();
    }
}

/* We may want to profile these to see if they're better off `bool`s. I'm only doing this for uniformity */

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum BasicTouchCombo {
    #[derivative(Default)]
    Inactive,
    Active,
}

impl Buff for BasicTouchCombo {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl DurationalBuff for BasicTouchCombo {
    const BASE_DURATION: u8 = 1;

    fn activate(self, _: u8) -> Self {
        Self::Active
    }
}

impl Sub<u8> for BasicTouchCombo {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");
        Self::Inactive
    }
}

impl SubAssign<u8> for BasicTouchCombo {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum ObservationCombo {
    #[derivative(Default)]
    Inactive,
    Active,
}

impl Buff for ObservationCombo {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl DurationalBuff for ObservationCombo {
    const BASE_DURATION: u8 = 1;

    fn activate(self, _: u8) -> Self {
        Self::Active
    }
}

impl Sub<u8> for ObservationCombo {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");
        Self::Inactive
    }
}

impl SubAssign<u8> for ObservationCombo {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}
