//! Contains buffs that modify the effects of actions on the `durability` attribute of the crafting state, such as [`WasteNot`].

use std::ops::{Sub, SubAssign};

use derivative::Derivative;

use super::{Buff, DurationalBuff};

/// A simple collection of all the durability buffs, for cleaner fields on simulation
/// structs.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
#[allow(missing_docs)]
pub struct DurabilityBuffs {
    pub manipulation: Manipulation,
    pub waste_not: WasteNot,
}

impl DurabilityBuffs {
    /// Causes all durational quality buffs to tick down by one.
    pub fn decay(&mut self) {
        self.manipulation.decay_in_place();
        self.waste_not.decay_in_place();
    }

    /// Returns the repair value of the current buffs (currently just [`Manipulation`]),
    /// which is applied after the item has been checked for failure or success.
    pub fn repair(&self) -> i8 {
        self.manipulation.repair()
    }

    /// Returns the modifier applied to durability (before dividing by 100) that will
    /// be applied to any action's durability cost.
    pub fn durability_cost_mod(&self) -> u16 {
        self.waste_not.durability_cost_mod()
    }
}

/// The buff associated with the action [Manipulation],
/// which repairs 5 durability onto the item after every action
/// (except for time-stopping ones),  as long as the item hasn't been broken or completed.
///
/// [`Manipulation`]: crate::actions::buffs::Manipulation
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum Manipulation {
    /// This buff is currently not active and gives no benefit.
    #[derivative(Default)]
    Inactive,
    /// This buff is active and will repair the item every time step
    Active(
        /// The number of turns remaining on this buff, once it hits
        /// 0 this will become [`Inactive`].
        ///
        /// [`Inactive`]: Manipulation::Inactive
        u8,
    ),
}

impl Manipulation {
    /// The amount of durability this repairs at the end of the turn
    pub const REPAIR_VALUE: i8 = 5;

    /// Returns the repair value only if the buff is currently [`Active`].
    ///
    /// [`Active`]: Manipulation::Active
    pub fn repair(self) -> i8 {
        match self {
            Self::Active(_) => Self::REPAIR_VALUE,
            Self::Inactive => 0,
        }
    }
}

impl Buff for Manipulation {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for Manipulation {
    const BASE_DURATION: u8 = 3;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl Sub<u8> for Manipulation {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(0) => unreachable!(),
            Self::Active(1) => Self::Inactive,
            Self::Active(val @ 2..) => Self::Active(val - rhs),
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for Manipulation {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

// Waste Not II overwrites Waste Not and vice versa when used, so makes sense in code to just consider
// Waste Not II as Waste Not with a +4 duration bonus.

/// The buff associated with the actions [`WasteNot`] and [`WasteNot2`]. In FFXIV
/// these are considered different buffs that overwrite each other, but instead we just
/// activate based on the duration passed in and consider it a single buff for code
/// simplicity.
///
/// [`WasteNot`]: crate::actions::buffs::WasteNot
/// [`WasteNot2`]: crate::actions::buffs::WasteNot2
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum WasteNot {
    /// This buff is currently not active and gives no benefit.
    #[derivative(Default)]
    Inactive,
    /// This buff was activated by the action [`WasteNot`] and is giving
    /// its benefit.
    ///
    /// [`WasteNot`]: crate::actions::buffs::WasteNot
    WasteNot(
        /// The number of turns remaining on this buff, once it hits
        /// 0 this will become [`Inactive`].
        ///
        /// [`Inactive`]: WasteNot::Inactive
        u8,
    ),

    /// This buff was activated by the action [`WasteNot2`] and is giving
    /// its benefit.
    ///
    /// [`WasteNot2`]: crate::actions::buffs::WasteNot2
    WasteNot2(
        /// The number of turns remaining on this buff, once it hits
        /// 0 this will become [`Inactive`].
        ///
        /// [`Inactive`]: WasteNot::Inactive
        u8,
    ),
}

impl WasteNot {
    /// The raw discount applied to the durability, before dividing by 100.
    pub const DISCOUNT: u16 = 50;

    /// Returns 100 (i.e. no chance) if [`WasteNot`] is [`Inactive`], otherwise
    /// returns the discount while the buff is active.
    ///
    /// [`Inactive`]: WasteNot::Inactive
    pub fn durability_cost_mod(self) -> u16 {
        match self {
            Self::WasteNot(_) | Self::WasteNot2(_) => Self::DISCOUNT,
            Self::Inactive => 100,
        }
    }
}

impl Buff for WasteNot {
    fn is_active(&self) -> bool {
        matches!(self, Self::WasteNot(_) | Self::WasteNot2(_))
    }
}

impl DurationalBuff for WasteNot {
    const BASE_DURATION: u8 = 4;

    fn activate(self, bonus: u8) -> Self {
        // 0+2 is meant to be descriptive of "base Waste Not + Primed"
        #[allow(clippy::identity_op)]
        if bonus == 0 || bonus == 0 + 2 {
            Self::WasteNot(Self::BASE_DURATION + bonus)
        } else if bonus == 4 || bonus == 4 + 2 {
            Self::WasteNot2(Self::BASE_DURATION + bonus)
        } else {
            panic!("Bonus duration must be 0, or any combination of `Primed` (+2) and `WasteNot2` (+4)")
        }
    }
}

impl Sub<u8> for WasteNot {
    type Output = Self;

    fn sub(mut self, rhs: u8) -> Self::Output {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::WasteNot(0) | Self::WasteNot2(0) => unreachable!(),
            Self::WasteNot(1) | Self::WasteNot2(1) => Self::Inactive,
            Self::WasteNot(ref mut val @ 2..) | Self::WasteNot2(ref mut val @ 2..) => {
                *val -= 1;
                self
            }
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for WasteNot {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs);
    }
}
