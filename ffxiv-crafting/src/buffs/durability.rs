//! Contains buffs that modify the effects of actions on the `durability` attribute of the crafting state, such as [`WasteNot`].

use std::ops::{Sub, SubAssign};

use derivative::Derivative;

use super::{Buff, DurationalBuff};

/// A simple collection of all the durability buffs, for cleaner fields on simulation
/// structs.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct DurabilityBuffs {
    pub manipulation: Manipulation,
    pub waste_not: WasteNot,
}

impl DurabilityBuffs {
    pub fn decay(&mut self) {
        self.manipulation.decay_in_place();
        self.waste_not.decay_in_place();
    }

    pub fn repair(&self) -> i8 {
        self.manipulation.repair()
    }

    pub fn durability_cost_mod(&self) -> u16 {
        self.waste_not.durability_cost_mod()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum Manipulation {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

impl Manipulation {
    const REPAIR_VALUE: i8 = 5;

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
            Self::Active(val) => Self::Active(val - rhs),
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum WasteNot {
    #[derivative(Default)]
    Inactive,
    WasteNot(u8),
    WasteNot2(u8),
}

impl WasteNot {
    const DISCOUNT: u16 = 50;

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
            Self::WasteNot(ref mut val) | Self::WasteNot2(ref mut val) => {
                *val -= 1;
                self
            }
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for WasteNot {
    fn sub_assign(&mut self, rhs: u8) {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::WasteNot(ref mut val) | Self::WasteNot2(ref mut val) => {
                *val -= 1;
            }
            _ => {}
        }
    }
}
