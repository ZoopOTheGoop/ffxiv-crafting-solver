//! Contains buffs that modify the effects of actions on the `quality` attribute of the crafting state, such as [`InnerQuiet`].

use derivative::Derivative;

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::buffs::{Buff, ConsumableBuff, DurationalBuff};

/// The max number of stacks [`InnerQuiet`] can have.
const MAX_IQ: u8 = 11;

/// A simple collection of all the quality buffs, for cleaner fields on simulation
/// structs.
#[allow(missing_docs)]
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct QualityBuffs {
    pub inner_quiet: InnerQuiet,
    pub great_strides: GreatStrides,
    pub innovation: Innovation,
}

impl QualityBuffs {
    /// Causes all durational quality buffs to tick down by one.
    pub fn decay(&mut self) {
        self.great_strides.decay_in_place();
        self.innovation.decay_in_place();
    }

    /// Calculates the efficiency bonuses granted by these buffs.
    pub fn efficiency_mod(&self) -> u16 {
        self.great_strides.efficiency_mod() + self.innovation.efficiency_mod()
    }
}

/// A trait that denotes something that affects quality. Largely just a marker trait
/// to denote intent.
pub trait QualityEfficiencyMod: DurationalBuff {
    /// The quality modifier, as internally defined. This is a percentage
    /// represented as an integer (i.e. 100 = 100% = 2x bonus).
    const MODIFIER: u16;

    /// Returns the efficiency modifier if the buff is currently active.
    fn efficiency_mod(&self) -> u16 {
        if self.is_active() {
            Self::MODIFIER
        } else {
            0
        }
    }
}

/// The Inner Quiet buff, when active, provides a 10% efficiency modifier per stack as
/// well as allowing the use of [`ByregotsBlessing`], to which it grants a further 20%
/// efficiency bonus per stack.
///
/// This implements [`Add`], [`Mul`], and [`Div`] to account for the abilities that have
/// these effects on Inner Quiet stacks. To have any effect, `activate` must be called
/// first (so you can safely apply effects to an inactive IQ and get an inactive IQ without
/// making your logic too ugly).
///
/// [`Reflect`]: crate::actions::quality::Reflect
/// [`ByregotsBlessing`]: crate::actions::quality::ByregotsBlessing
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum InnerQuiet {
    /// This buff is current not active and gives no benefit.
    #[derivative(Default)]
    Inactive,
    /// This buff is active and will apply its modifier to its
    /// associated actions.
    Active(
        /// The number of stacks of [`InnerQuiet`], up to a max
        /// of 10.
        ///
        /// This provides a 20% efficiency bonus to [`ByregotsBlessing`].
        ///
        /// Upon using [`ByregotsBlessing`] this buff will be [consumed],
        /// changing it back to [`Inactive`].
        ///
        /// [`ByregotsBlessing`]: crate::actions::quality::ByregotsBlessing
        /// [consumed]: ConsumableBuff
        /// [`Inactive`]: InnerQuiet::Inactive
        u8,
    ),
}

impl InnerQuiet {
    /// Returns the [`InnerQuiet`] quality modifier, which is 20% per stack added on to
    /// the character's current `control`.
    pub fn quality_mod(&self, control: u16) -> f64 {
        match self {
            Self::Active(stacks) => {
                debug_assert_ne!(*stacks, 0);
                debug_assert!(*stacks <= 10);

                control as f64 + control as f64 * ((*stacks as f64 - 1.) * 20. / 100.)
            }
            // Needs testing, hard to tell if this should be 0, `control`, or -20% from
            // the doc
            Self::Inactive => control as f64,
        }
    }

    /// Retrieves the number of stacks.
    ///
    /// [`Inactive`]: InnerQuiet::Inactive
    pub fn stacks(&self) -> u8 {
        match self {
            Self::Active(val) => *val,
            Self::Inactive => 0,
        }
    }

    /// Returns the additive bonus to efficiency granted by inner quiet
    pub fn efficiency_bonus(&self) -> u16 {
        (self.stacks() as u16) * 10
    }
}

impl Buff for InnerQuiet {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl ConsumableBuff for InnerQuiet {
    fn deactivate(self) -> (Self, u8) {
        match self {
            Self::Active(stacks) => (Self::Inactive, stacks),
            Self::Inactive => panic!("Attempt to deactivate inactive IQ buff, check logic"),
        }
    }
}

impl Add<u8> for InnerQuiet {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        debug_assert!(
            rhs == 1 || rhs == 2,
            "Should only add 1 or 2 to Inner Quiet"
        );
        match self {
            Self::Inactive => Self::Inactive,
            Self::Active(stacks) => Self::Active((stacks + rhs).min(MAX_IQ)),
        }
    }
}

impl AddAssign<u8> for InnerQuiet {
    fn add_assign(&mut self, rhs: u8) {
        *self = self.add(rhs)
    }
}

impl Mul<u8> for InnerQuiet {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        match self {
            Self::Inactive => Self::Inactive,
            Self::Active(stacks) => Self::Active((stacks * rhs).min(MAX_IQ)),
        }
    }
}

impl MulAssign<u8> for InnerQuiet {
    fn mul_assign(&mut self, rhs: u8) {
        *self = self.mul(rhs)
    }
}

impl Div<u8> for InnerQuiet {
    type Output = Self;

    fn div(self, rhs: u8) -> Self::Output {
        match self {
            Self::Inactive => Self::Inactive,
            Self::Active(stacks) => Self::Active((stacks / rhs).min(MAX_IQ)),
        }
    }
}

impl DivAssign<u8> for InnerQuiet {
    fn div_assign(&mut self, rhs: u8) {
        *self = self.div(rhs)
    }
}

/// The buff associated with the action [GreatStrides],
/// which adds 1.0 to the multiplier on the efficiency of the next [`quality`]
/// related actions (the base multiplier is 1.0, so this changes it to 2.0, not including
/// other buffs).
///
/// [`quality`]: crate::actions::quality
/// [`GreatStrides`]: crate::actions::buffs::GreatStrides
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum GreatStrides {
    /// This buff is currently not active and gives no benefit.
    #[derivative(Default)]
    Inactive,
    /// This buff is active and will apply its modifier to its
    /// associated actions.
    Active(
        /// The number of turns remaining on this buff, once it hits
        /// 0 this will become [`Inactive`]. As this is a [`ConsumableBuff`],
        /// this will also become [`Inactive`] if its trigger is hit.
        ///
        /// [`Inactive`]: GreatStrides::Inactive
        u8,
    ),
}

impl Buff for GreatStrides {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for GreatStrides {
    const BASE_DURATION: u8 = 3;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl ConsumableBuff for GreatStrides {
    fn deactivate(self) -> (Self, u8) {
        match self {
            Self::Active(duration) => (Self::Inactive, duration),
            Self::Inactive => panic!("Attempt to consume Great Strides when it's not active"),
        }
    }
}

impl QualityEfficiencyMod for GreatStrides {
    const MODIFIER: u16 = 100;
}

impl Sub<u8> for GreatStrides {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Inactive | Self::Active(1) => Self::Inactive,
            Self::Active(val) => Self::Active(val - rhs),
        }
    }
}

impl SubAssign<u8> for GreatStrides {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

/// The buff associated with the action [`Innovation`],
/// which adds 0.5 to the multiplier on the efficiency of the next [`quality`]
/// related actions (the base multiplier is 1.0, so this changes it to 1.5, not including
/// other buffs).
///
/// [`quality`]: crate::actions::quality
/// [`Innovation`]: crate::actions::buffs::Innovation
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum Innovation {
    /// This buff is currently not active and gives no benefit.
    #[derivative(Default)]
    Inactive,
    /// This buff is active and will apply its modifier to its
    /// associated actions.
    Active(
        /// The number of turns remaining on this buff, once it hits
        /// 0 this will become [`Inactive`].
        ///
        /// [`Inactive`]: Innovation::Inactive
        u8,
    ),
}

impl Buff for Innovation {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for Innovation {
    const BASE_DURATION: u8 = 4;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl QualityEfficiencyMod for Innovation {
    const MODIFIER: u16 = 50;
}

impl Sub<u8> for Innovation {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Inactive | Self::Active(1) => Self::Inactive,
            Self::Active(val) => Self::Active(val - rhs),
        }
    }
}

impl SubAssign<u8> for Innovation {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}
