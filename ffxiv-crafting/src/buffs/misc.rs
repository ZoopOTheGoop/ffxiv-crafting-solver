//! Contains some miscellaneous things that go in the buff struct for convenience.

use std::ops::{Sub, SubAssign};

use derivative::Derivative;

/// Denotes the number of crafters' delineations the character has left, if any, or
/// if they're not a specialist. This is solely useful for the [`CarefulObservation`]
/// action.
///
/// This implements [`Sub`] for subtracting delineations one at a time.
///
/// [`CarefulObservation`]: crate::actions::misc::CarefulObservation
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Derivative)]
#[derivative(Default)]
pub enum SpecialistActions {
    #[derivative(Default)]
    NotSpecialist,
    Unavailable,
    Availalble(u8),
}

impl SpecialistActions {
    /// Returns if specialist actions can be used (i.e. there are enough delineations).
    pub fn actions_available(&self) -> bool {
        matches!(self, Self::Availalble(_))
    }

    /// Returns if specialist actions cannot be used (i.e. no delineations or not a specialist).
    pub fn actions_unavailable(&self) -> bool {
        matches!(self, Self::Unavailable | Self::NotSpecialist)
    }
}

impl Sub<u8> for SpecialistActions {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        debug_assert_eq!(rhs, 1, "Action should only use one delineation at a time");

        #[cfg(debug_assertions)]
        match self {
            Self::Availalble(0) => {
                panic!("Specialist Actions shouldn't be listed as available with 0 delineations")
            }
            Self::Availalble(4..=u8::MAX) => {
                panic!("Too many crafters delineations - we're constrained to 3 per craft.")
            }
            Self::Availalble(val @ 1..=3) => Self::Availalble(val - 1),
            Self::NotSpecialist => Self::NotSpecialist,
            Self::Unavailable => Self::Unavailable,
        }

        #[cfg(not(debug_assertions))]
        match self {
            Self::Availalble(0..=1) => Self::Unavailable,
            Self::Availalble(val) => Self::Availalble(val - 1),
            Self::NotSpecialist => Self::NotSpecialist,
            Self::Unavailable => Self::Unavailable,
        }
    }
}

impl SubAssign<u8> for SpecialistActions {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}
