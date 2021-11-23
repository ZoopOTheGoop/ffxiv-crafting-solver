//! Contains buffs that modify the effects of actions on the `progress` attribute of the crafting state, such as [`Veneration`].

use std::ops::{Sub, SubAssign};

use derivative::Derivative;

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

use super::{Buff, BuffState, ConsumableBuff, DurationalBuff};

/// A simple collection of all the progress buffs, for cleaner fields on simulation
/// structs.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct ProgressBuffs {
    pub name_of_the_elements: NameOfTheElements,
    pub veneration: Veneration,
    pub muscle_memory: MuscleMemory,
    pub final_appraisal: FinalAppraisal,
}

impl ProgressBuffs {
    pub fn decay(&mut self) {
        self.name_of_the_elements.decay_in_place();
        self.veneration.decay_in_place();
        self.muscle_memory.decay_in_place();
        self.final_appraisal.decay_in_place();
    }

    /// Calculates the efficiency bonuses granted by these buffs. This does NOT include the [`NameOfTheElements`] buff,
    /// as it's overridden specially by its matching action [`BrandOfTheElements`].
    pub fn efficiency_mod(&self) -> u16 {
        self.veneration.efficiency_mod() + self.muscle_memory.efficiency_mod()
    }
}

/// A trait that denotes something that affects quality. Largely just a marker trait
/// to denote intent. This doesn't work for [`NameOfTheElements`], as that only has its
/// efficiency effect when used with [`BrandOfTheElements`] and requires extra information.
pub trait ProgressEfficiencyMod: DurationalBuff {
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum NameOfTheElements {
    #[derivative(Default)]
    Unused,
    Active(u8),
    Used,
}

impl NameOfTheElements {
    pub fn can_activate(&self) -> bool {
        matches!(self, Self::Unused)
    }

    pub fn already_activated(&self) -> bool {
        matches!(self, Self::Active(_) | Self::Used)
    }
}

impl Buff for NameOfTheElements {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for NameOfTheElements {
    const BASE_DURATION: u8 = 3;

    fn activate(self, bonus: u8) -> Self {
        match self {
            Self::Unused => Self::Active(Self::BASE_DURATION + bonus),
            Self::Active(_) | Self::Used => {
                panic!("Cannot activate Name of the Elements twice, check logic.")
            }
        }
    }
}

impl Sub<u8> for NameOfTheElements {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(val) => Self::Active(val - rhs),
            Self::Unused => Self::Unused,
            Self::Used => Self::Used,
        }
    }
}

impl SubAssign<u8> for NameOfTheElements {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum Veneration {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

impl Buff for Veneration {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl DurationalBuff for Veneration {
    const BASE_DURATION: u8 = 4;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl Sub<u8> for Veneration {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(val) => Self::Active(val - rhs),
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for Veneration {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

impl ProgressEfficiencyMod for Veneration {
    const MODIFIER: u16 = 50;
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum MuscleMemory {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

impl Buff for MuscleMemory {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl ConsumableBuff for MuscleMemory {
    fn deactivate(self) -> (Self, u8) {
        match self {
            Self::Active(val) => (Self::Inactive, val),
            Self::Inactive => panic!("Attempt to deactivate inactive Muscle Memory"),
        }
    }
}

impl DurationalBuff for MuscleMemory {
    const BASE_DURATION: u8 = 5;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl Sub<u8> for MuscleMemory {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(val) => Self::Active(val - rhs),
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for MuscleMemory {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}

impl ProgressEfficiencyMod for MuscleMemory {
    const MODIFIER: u16 = 100;
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
pub enum FinalAppraisal {
    #[derivative(Default)]
    Inactive,
    Active(u8),
}

impl FinalAppraisal {
    pub fn handle_progress<C, M>(
        &self,
        state: &CraftingState<C, M>,
        new_progress: u32,
        buffs: &mut BuffState,
    ) -> u32
    where
        C: Condition,
        M: QualityMap,
    {
        if self.is_active()
            && state.curr_progress + new_progress >= state.recipe.recipe.max_progress
        {
            buffs.progress.final_appraisal.deactivate();
            (state.recipe.recipe.max_progress - 1) - state.curr_progress
        } else {
            new_progress
        }
    }
}

impl Buff for FinalAppraisal {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

impl ConsumableBuff for FinalAppraisal {
    fn deactivate(self) -> (Self, u8) {
        match self {
            Self::Active(val) => (Self::Inactive, val),
            Self::Inactive => panic!("Attempt to deactivate inactive Final Appraisal"),
        }
    }
}

impl DurationalBuff for FinalAppraisal {
    const BASE_DURATION: u8 = 5;

    fn activate(self, bonus: u8) -> Self {
        Self::Active(Self::BASE_DURATION + bonus)
    }
}

impl Sub<u8> for FinalAppraisal {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self {
        debug_assert_eq!(rhs, 1, "Buffs should only decrease their duration by 1");

        match self {
            Self::Active(val) => Self::Active(val - rhs),
            Self::Inactive => Self::Inactive,
        }
    }
}

impl SubAssign<u8> for FinalAppraisal {
    fn sub_assign(&mut self, rhs: u8) {
        *self = self.sub(rhs)
    }
}
