//! Contains some miscellaneous things that go in the buff struct for convenience.

use std::ops::{Sub, SubAssign};

use super::{Buff, ConsumableBuff};

/// Denotes the number of crafters' delineations the character has left, if any, or
/// if they're not a specialist. This is solely useful for the [`CarefulObservation`]
/// and [`HeartAndSoul`] actions.
///
/// This implements [`Sub`] for subtracting delineations one at a time.
///
/// [`CarefulObservation`]: crate::actions::misc::CarefulObservation
/// [`HeartAndSoul`]: crate::actions::buffs::HeartAndSoul
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Default)]
pub enum SpecialistActions {
    /// The crafter is not a specialist.
    #[default]
    NotSpecialist,
    /// The crafter is a specialist, but has no delineations (or
    /// has used all 3 allowed charges already).
    Unavailable,
    /// The crafter has crafters delineations that are still allowed to be used.
    Availalble(
        /// The number of delineations - from 1-3. At 0 this will change to [`Unavailable`]
        ///
        /// [`Unavailable`]: SpecialistActions::Unavailable
        u8,
    ),
}

impl SpecialistActions {
    /// Returns if specialist actions can be used (i.e. there are enough delineations).
    pub fn actions_available(&self) -> bool {
        self.verify_state();
        matches!(self, Self::Availalble(_))
    }

    /// Returns if specialist actions cannot be used (i.e. no delineations or not a specialist).
    pub fn actions_unavailable(&self) -> bool {
        self.verify_state();
        matches!(self, Self::Unavailable | Self::NotSpecialist)
    }

    #[inline(always)]
    fn verify_state(&self) {
        debug_assert!(
            !matches!(self, Self::Availalble(4..)),
            "Too many delineations {:?}; should have at most 3",
            self
        );
        debug_assert!(
            !matches!(self, Self::Availalble(0)),
            "Shouldn't be marked as available with 0 delineations",
        );
    }
}

impl Sub<u8> for SpecialistActions {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        debug_assert_eq!(rhs, 1, "Action should only use one delineation at a time");

        self.verify_state();

        match self {
            Self::Availalble(1) => Self::Unavailable,
            Self::Availalble(val @ 2..=3) => Self::Availalble(val - 1),
            Self::NotSpecialist => Self::NotSpecialist,
            Self::Unavailable => Self::Unavailable,
            _ => unreachable!(),
        }
    }
}

impl SubAssign<u8> for SpecialistActions {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

/// The buff associated with the action [`HeartAndSoul`],
/// which allows actions such as [`TricksOfTheTrade`] to be executed even when the
/// condition is not good or excellent.
///
/// [`quality`]: crate::actions::quality
/// [`HeartAndSoul`]: crate::actions::buffs::HeartAndSoul
/// [`TricksOfTheTrade`]: crate::actions::misc::TricksOfTheTrade
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub enum HeartAndSoul {
    /// This buff is currently not active and gives no benefit.
    #[default]
    Inactive,
    /// This buff is active and will apply its modifier to its
    /// associated actions.
    Active,
}

impl HeartAndSoul {
    /// Activates this buff. Similar to the variant in [`DurationalBuff`], but this buff
    /// does not have a duration.
    ///
    /// [`DurationalBuff`]: crate::buffs::DurationalBuff
    pub fn activate(self) -> Self {
        Self::Active
    }

    /// Activates this buff and overwrites the current value.
    /// Similar to the variant in [`DurationalBuff`], but this buff does not have a duration.
    ///
    /// [`DurationalBuff`]: crate::buffs::DurationalBuff
    pub fn activate_in_place(&mut self) {
        *self = Self::Active
    }
}

impl Buff for HeartAndSoul {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl ConsumableBuff for HeartAndSoul {
    fn deactivate(self) -> (Self, u8) {
        debug_assert_eq!(
            self,
            HeartAndSoul::Active,
            "Attempted to deactivate deactivated HeartAndSoul"
        );
        (Self::Inactive, u8::MAX)
    }
}
