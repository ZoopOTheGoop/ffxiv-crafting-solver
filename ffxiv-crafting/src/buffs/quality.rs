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
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct QualityBuffs {
    pub inner_quiet: InnerQuiet,
    pub great_strides: GreatStrides,
    pub innovation: Innovation,
}

// We could make `InnerQuiet` a "StackingBuff" and "QualityModBuff", but it's such a special case tbh,
// maybe if more buffs of that form get added. It wouldn't exactly be a difficult refactor.

/// The number of stacks to initialize [`InnerQuiet`] at, depending on the ability used.
/// #[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum InnerQuietBaseStacks {
    InnerQuiet = 1,
    Reflection = 4,
}

/// The Inner Quiet buff, when active, provides a 20% quality modifier per stack.
///
/// This implements [`Add`], [`Mul`], and [`Div`] to account for the abilities that have
/// these effects on Inner Quiet stacks. To have any effect, `activate` must be called
/// first (so you can safely apply effects to an inactive IQ and get an inactive IQ without
/// making your logic too ugly).
///
/// This is a very unique type of buff in logic, so it doesn't implement many buff traits,
/// as no other current buffs operate off stacks nor do they have multiple activation
/// modes ([`Reflection`] vs the action [`InnerQuiet`]() itself) that can't be done if
/// already active.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum InnerQuiet {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

impl InnerQuiet {
    /// Returns the [`InnerQuiet`] quality modifier, which is 20% per stack added on to
    /// the character's current `control`.
    pub fn quality_mod(&self, control: u16) -> f64 {
        match self {
            Self::Active(stacks) => {
                debug_assert_ne!(*stacks, 0);
                debug_assert!(*stacks <= 11);

                control as f64 + control as f64 * ((*stacks as f64 - 1.) * 20. / 100.)
            }
            // Needs testing, hard to tell if this should be 0, `control`, or -20% from
            // the doc
            Self::Inactive => control as f64,
        }
    }

    /* Arguably these could be `Option` or `Result`, but IMO your program is wrong if you try
    to activate/deactivate at the wrong time. If you need to switch use `match` or `is_active` */

    /// Activates the [`InnerQuiet`] buff, to the value indicated by `base`.
    pub fn activate(self, base: InnerQuietBaseStacks) -> Self {
        match self {
            Self::Inactive => Self::from(base),
            Self::Active(_) => panic!("Attempt to activate already active IQ buff, check logic"),
        }
    }

    /// Mutates the current value instead of returning a new one for convenience. Essentially the
    /// equivalent of [`SubAssign`] for `activate`.
    pub fn activate_in_place(&mut self, base: InnerQuietBaseStacks) {
        *self = self.activate(base)
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
            Self::Inactive => panic!("Attempt to deactivate active IQ buff, check logic"),
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

impl From<InnerQuietBaseStacks> for InnerQuiet {
    fn from(base: InnerQuietBaseStacks) -> Self {
        Self::Active(base as u8)
    }
}

/// A trait that denotes something that affects quality. Largely just a marker trait
/// to denote intent.
pub trait QualityEfficiencyMod: DurationalBuff {
    /// The quality modifier, as internally defined. This is a percentage
    /// represented as an integer (i.e. 100 = 100% = 2x bonus).
    const MODIFIER: u16;

    /// Returns the efficiency modifier if the buff is currently active, otherwise 0.
    ///
    /// The default impl simply defers to [`Buff::is_active`].
    fn efficiency_mod(&self) -> u16 {
        if self.is_active() {
            Self::MODIFIER
        } else {
            0
        }
    }
}

/// The Great Strides buff, which adds 100% efficiency onto the next quality action.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum GreatStrides {
    #[derivative(Default)]
    Inactive,
    Active(u8),
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

/// The Innovation buff, which adds 50% efficiency onto the next quality action.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum Innovation {
    #[derivative(Default)]
    Inactive,
    Active(u8),
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