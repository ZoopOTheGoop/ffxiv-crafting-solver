//! Fake "buffs" that are really triggers for combo actions. [`MuscleMemory`] remains grouped with the [`progress`]
//! buffs because it's more appropriate there.
//!
//! [`MuscleMemory`]: crate::buffs::progress::MuscleMemory
//! [`progress`]: crate::buffs::progress

use super::{Buff, DurationalBuff};

/// A collection of miscellaneous combo triggers that don't fit elsewhere.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
#[allow(missing_docs)]
pub struct ComboTriggers {
    pub basic_touch: BasicTouchCombo,
    pub observation: ObserveCombo,
}

impl ComboTriggers {
    /// On normal buffs like [`QualityBuffs`], this would
    /// cause all durational quality buffs to tick down by one.
    ///
    /// These combo actions are implemented like those buffs, but they only last
    /// for one action (as long as their combo), so it just turns them off if they're
    /// on.
    ///
    /// Note that unlike other buffs, this is called when [time stopping] actions are used,
    /// as they still invalidate combos.
    ///
    /// [`QualityBuffs`]: crate::buffs::quality::QualityBuffs
    /// [time stopping]: crate::actions::TimePassing
    pub fn decay(&mut self) {
        self.basic_touch.decay_in_place();
        self.observation.decay_in_place();
    }
}

/// Denotes the combo between [`BasicTouch`], [`StandardTouch`], and [`AdvancedTouch`]. Three moves
/// that, when used after the previous one in the chain, inherit the low CP cost of [`BasicTouch`], but
/// with increasing efficiency.
///
/// [`BasicTouch`]: crate::actions::quality::BasicTouch
/// [`StandardTouch`]: crate::actions::quality::StandardTouch
/// [`AdvancedTouch`]: crate::actions::quality::StandardTouch
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum BasicTouchCombo {
    /// [`BasicTouch`] was not used last turn and its combo is unavailable.
    ///
    /// [`BasicTouch`]: crate::actions::quality::BasicTouch
    #[default]
    Inactive,

    /// [`BasicTouch`] was used last turn and its combo is available.
    ///
    /// [`BasicTouch`]: crate::actions::quality::BasicTouch
    BasicTouch,

    /// [`StandardTouch`] was used last turn after [`BasicTouch`] and its combo is available.
    ///
    /// [`BasicTouch`]: crate::actions::quality::BasicTouch
    /// [`StandardTouch`]: crate::actions::quality::StandardTouch
    StandardTouch,
}

impl Buff for BasicTouchCombo {
    fn is_active(&self) -> bool {
        matches!(self, Self::BasicTouch | Self::StandardTouch)
    }
}

impl DurationalBuff for BasicTouchCombo {
    const BASE_DURATION: u8 = 1;

    fn activate(self, _: u8) -> Self {
        Self::BasicTouch
    }

    fn decay(self) -> Self {
        Self::Inactive
    }

    fn remaining_duration(&self) -> Option<u8> {
        match *self {
            Self::Inactive => None,
            _ => Some(1),
        }
    }
}

/// Denotes that [`Observe`] was used last turn, and thus [`PatientTouch`] and
/// [`FocusedSynthesis`] have their success rates increased to 100.
///
/// [`Observe`]: crate::actions::misc::Observe
/// [`PatientTouch`]: crate::actions::quality::PatientTouch
/// [`FocusedSynthesis`]: crate::actions::progress::FocusedSynthesis
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ObserveCombo {
    /// [`Observe`] was not used last turn and its combo is unavailable.
    ///
    /// [`Observe`]: crate::actions::misc::Observe
    #[default]
    Inactive,

    /// [`Observe`] was used last turn and its combo is available.
    ///
    /// [`Observe`]: crate::actions::misc::Observe
    Active,
}

impl Buff for ObserveCombo {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl DurationalBuff for ObserveCombo {
    const BASE_DURATION: u8 = 1;

    fn activate(self, _: u8) -> Self {
        Self::Active
    }

    fn decay(self) -> Self {
        Self::Inactive
    }

    fn remaining_duration(&self) -> Option<u8> {
        match *self {
            Self::Inactive => None,
            _ => Some(1),
        }
    }
}
