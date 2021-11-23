//! Contains general trait definitions for crafting buffs used across the simulator.

use std::ops::{Sub, SubAssign};

use crate::actions::misc::SpecialistActions;

use self::{
    combo::ComboTriggers, durability::DurabilityBuffs, progress::ProgressBuffs,
    quality::QualityBuffs,
};

pub mod combo;
pub mod durability;
pub mod progress;
pub mod quality;

/// The basic form of a crafting buff. Since buffs can be a lot of different forms, the only
/// thing they really have in common is the fact that they can be inactive or active.
///
/// Most buffs will implement more refined buff traits such as [`DurationalBuff`], or are
/// such special cases like as [`InnerQuiet`] that it doesn't pay to encode the meaning
/// in a trait.
///
/// [`InnerQuiet`]: self::quality::InnerQuiet
pub trait Buff: Copy + Sized {
    /// Checks whether the current buff is active or not.
    fn is_active(&self) -> bool;

    /// Checks whether the current buff is inactive or not, default implementation
    /// just negates the value of [`is_active`](Buff::is_active).
    fn is_inactive(&self) -> bool {
        !self.is_active()
    }
}

/// A [`Buff`] which is only active for a fixed amount of time after activation. This should
/// implement [`Sub`], and expect that it will only ever decay by 1 (i.e. a turn), and panic otherwise.
/// This can be revisited if things such as actions or expert conditions that more quickly decay buffs
/// are added later.
///
/// When a buff is [`activate`](DurationalBuff::activate)d, it should set its duration to the value of
/// [`BASE_DURATION`](DurationalBuff::BASE_DURATION), plus any bonuses it receives (e.g. from the
/// [`Primed`] condition), regardless of whether it's already active or not.
///
/// Note: for buffs whose duration can "decay" all at once under certain conditions,
/// such as [`GreatStrides`], you're looking for [`ConsumableBuff`] either instead of or in addition
/// to this.
///
/// [`GreatStrides`]: crate::actions::quality::buffs::GreatStrides
pub trait DurationalBuff: Sub<u8, Output = Self> + SubAssign<u8> + Buff {
    /// The length that this buff will be active for when triggered by an action,
    /// before any condition or other modifiers.
    const BASE_DURATION: u8;

    /// Activates the buff, setting its duration to
    /// [`BASE_DURATION`](DurationalBuff::BASE_DURATION), plus the value of `bonus`.
    ///
    /// This should occur even if the buff is already active.
    fn activate(self, bonus: u8) -> Self;

    /// Mutates the value, replacing it with its activated form. This is essentially
    /// as to [`activate`](DurationalBuff::activate) and [`SubAssign`] is to [`Sub`].
    ///
    /// The default impl simply calls [`activate`](DurationalBuff::activate) and uses
    /// its return value.
    fn activate_in_place(&mut self, bonus: u8) {
        *self = self.activate(bonus)
    }

    /// A semantic wrapper over `self - 1`.
    fn decay(self) -> Self {
        self - 1
    }

    /// A semantic wrapper over `self -= 1`.
    fn decay_in_place(&mut self) {
        *self = self.decay()
    }
}

/// A buff that can be "consumed" by an action, such as [`InnerQuiet`] and [`ByregotsBlessing`],
/// or [`GreatStrides`] and any [quality action].
///
/// When consumed, the buff returns its remaining duration for use by the user. If the buff is
/// already inactive this should panic, as a well-formed implementation will not do that.
///
/// [`InnerQuiet`]: crate::actions::quality::buffs::InnerQuiet
/// [`GreatStrides`]: crate::actions::quality::buffs::GreatStrides
/// [quality action]: crate::actions::quality
pub trait ConsumableBuff: Buff {
    /// Returns a deactivated version of `self`,
    /// as well as any stacks or remaining duration. This should
    /// panic if the buff is already inactive, as a well-formed program
    /// will not call that.
    fn deactivate(self) -> (Self, u8);

    /// Mutates the value, replacing it with its deactivated form. This is essentially
    /// as to [`deactivate`](ConsumableBuff::deactivate) and [`SubAssign`] is to [`Sub`].
    ///
    /// The default impl simply calls [`deactivate`](ConsumableBuff::deactivate) and uses
    /// its return value.
    fn deactivate_in_place(&mut self) -> u8 {
        let (new, remaining) = self.deactivate();
        *self = new;
        remaining
    }
}

/// Encodes the buff state during crafting. Has several utility methods to make
/// buff management a bit less ugly
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct BuffState {
    pub quality: QualityBuffs,
    pub progress: ProgressBuffs,
    pub durability: DurabilityBuffs,
    pub combo: ComboTriggers,
    // This is honestly only in the buff state because I didn't want to add another
    // trait just for this
    pub specialist_actions: SpecialistActions,
}

impl BuffState {
    /// Makes all active buffs that "tick down" over time decay one step.
    pub fn decay(&mut self) {
        self.quality.decay();
        self.progress.decay();
        self.durability.decay();
        self.combo.decay();
    }
}
