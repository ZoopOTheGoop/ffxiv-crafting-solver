//! Contains buffs that modify the effects of actions on the `progress` attribute of the crafting state, such as [`Veneration`].

use crate::{conditions::Condition, quality_map::QualityMap, CraftingState};

use super::{Buff, BuffState, ConsumableBuff, DurationalBuff};
use ffxiv_crafting_derive::{Buff, ConsumableBuff, DurationalBuff};

/// A simple collection of all the progress buffs, for cleaner fields on simulation
/// structs.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
#[allow(missing_docs)]
pub struct ProgressBuffs {
    pub veneration: Veneration,
    pub muscle_memory: MuscleMemory,
    pub final_appraisal: FinalAppraisal,
}

impl ProgressBuffs {
    /// Causes all durational progress buffs to tick down by one.
    pub fn decay(&mut self) {
        self.veneration.decay_in_place();
        self.muscle_memory.decay_in_place();
        self.final_appraisal.decay_in_place();
    }

    /// Calculates the efficiency bonuses granted by these buffs.
    pub fn efficiency_mod(&self) -> u16 {
        self.veneration.efficiency_mod()
    }

    /// Calculates the efficiency added on to the next action, if any.
    pub fn bonus_efficiency(&self) -> u16 {
        self.muscle_memory.bonus_efficiency()
    }
}

/// A trait that denotes something that affects quality. Largely just a marker trait
/// to denote intent.
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

/// The buff state corresponding to the action [`Veneration`],
/// which adds 0.5 to the multiplier on the efficiency of [`progress`] related
/// actions (the base multiplier is 1.0, so this changes it to 1.5, not including
/// other buffs).
///
/// [`Veneration`]: crate::actions::buffs::Veneration
/// [`progress`]: crate::actions::progress
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Buff,
    DurationalBuff
)]
#[ffxiv(duration = 4)]
pub struct Veneration(pub(super) u8);

impl ProgressEfficiencyMod for Veneration {
    const MODIFIER: u16 = 50;
}

/// The buff state corresponding to the action [`MuscleMemory`],
/// which adds 100 to the base efficiency of the next [`progress`] related
/// action.
///
/// This is a "combo action", and is consumed once another [`progress`]
/// related action is used, consuming the buff.
///
/// [`MuscleMemory`]: crate::actions::progress::MuscleMemory
/// [`progress`]: crate::actions::progress
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Buff,
    DurationalBuff,
    ConsumableBuff
)]
#[ffxiv(duration = 5)]
pub struct MuscleMemory(pub(super) u8);

impl MuscleMemory {
    /// The bonus efficiency added onto the next synthesis action, when
    /// active.
    pub const BONUS: u16 = 100;

    /// Returns the bonus efficiency to be added on to the next
    /// synthesis action. 100 if active and 0 otherwise.
    fn bonus_efficiency(&self) -> u16 {
        if self.is_active() {
            Self::BONUS
        } else {
            0
        }
    }
}

/// The buff state corresponding to the action [`FinalAppraisal`],
/// which causes the next action by a [`progress`] related
/// actions that would finish the craft to leave it with 1 left instead.
///
/// [`FinalAppraisal`]: crate::actions::buffs::FinalAppraisal
/// [`progress`]: crate::actions::progress
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Buff,
    DurationalBuff,
    ConsumableBuff
)]
#[ffxiv(duration = 5)]
pub struct FinalAppraisal(pub(super) u8);

impl FinalAppraisal {
    /// Compares progress computed during the most recent
    /// action execution stage to the progress needed to
    /// finish the craft, returning the actual delta that must
    /// be applied to just barely not finish the craft and [consuming]
    /// the buff.
    ///
    /// [consuming]: ConsumableBuff
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
            && state.curr_progress + new_progress >= state.problem_def.recipe.max_progress
        {
            buffs.progress.final_appraisal.deactivate_in_place();
            (state.problem_def.recipe.max_progress - 1) - state.curr_progress
        } else {
            new_progress
        }
    }
}
