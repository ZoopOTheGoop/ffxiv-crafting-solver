//! Contains buffs that modify the effects of actions on the `quality` attribute of the crafting state, such as [`InnerQuiet`].

use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

use crate::buffs::{Buff, ConsumableBuff, DurationalBuff};
use ffxiv_crafting_derive::{Buff, ConsumableBuff, DurationalBuff};

/// The max number of stacks [`InnerQuiet`] can have.
const MAX_IQ: u8 = 10;

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
    pub fn efficiency_mod(&self) -> u32 {
        self.inner_quiet.efficiency_bonus()
            * (100 + self.great_strides.efficiency_mod() + self.innovation.efficiency_mod())
            / 100
    }
}

/// A trait that denotes something that affects quality. Largely just a marker trait
/// to denote intent.
pub trait QualityEfficiencyMod: DurationalBuff {
    /// The quality modifier, as internally defined. This is a percentage
    /// represented as an integer (i.e. 100 = 100% = 2x bonus).
    const MODIFIER: u32;

    /// Returns the efficiency modifier if the buff is currently active.
    fn efficiency_mod(&self) -> u32 {
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
#[derive(
    Clone,
    Copy,
    Hash,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Default,
    Buff,
    ConsumableBuff
)]
pub struct InnerQuiet(u8);

impl InnerQuiet {
    /// Retrieves the number of stacks.
    ///
    /// [`Inactive`]: InnerQuiet::Inactive
    pub fn stacks(&self) -> u8 {
        self.0
    }

    /// Returns the additive bonus to efficiency granted by inner quiet
    pub fn efficiency_bonus(&self) -> u32 {
        debug_assert!(
            self.0 <= MAX_IQ,
            "IQ stacks somehow exceeded max; {} > {}",
            self.0,
            MAX_IQ
        );

        100 + (self.stacks() as u32) * 10
    }
}

impl Add<u8> for InnerQuiet {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self((self.0 + rhs).min(10))
    }
}

impl AddAssign<u8> for InnerQuiet {
    fn add_assign(&mut self, rhs: u8) {
        *self = self.add(rhs)
    }
}

/// The buff associated with the action [GreatStrides],
/// which adds 1.0 to the multiplier on the efficiency of the next [`quality`]
/// related actions (the base multiplier is 1.0, so this changes it to 2.0, not including
/// other buffs).
///
/// [`quality`]: crate::actions::quality
/// [`GreatStrides`]: crate::actions::buffs::GreatStrides
#[derive(
    Clone,
    Copy,
    Hash,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Default,
    Buff,
    DurationalBuff,
    ConsumableBuff
)]
#[ffxiv(duration = 3)]
pub struct GreatStrides(pub(super) u8);

impl QualityEfficiencyMod for GreatStrides {
    const MODIFIER: u32 = 100;
}

/// The buff associated with the action [`Innovation`],
/// which adds 0.5 to the multiplier on the efficiency of the next [`quality`]
/// related actions (the base multiplier is 1.0, so this changes it to 1.5, not including
/// other buffs).
///
/// [`quality`]: crate::actions::quality
/// [`Innovation`]: crate::actions::buffs::Innovation
#[derive(
    Clone,
    Copy,
    Hash,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Default,
    Buff,
    DurationalBuff
)]
#[ffxiv(duration = 4)]
pub struct Innovation(pub(super) u8);

impl QualityEfficiencyMod for Innovation {
    const MODIFIER: u32 = 50;
}
