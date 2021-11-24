//! Provides collections containing all the actions for fast dispatch in simulators. These are shims that just defer to
//! the underlying implementation with as few branches as possible.

use super::{buffs::*, misc::*, progress::*, quality::*};

use ffxiv_crafting_derive::ffxiv_action_enum;

/// "A collection of all actions in FFXIV. They're organized by [`buffs`], then
/// [`misc`], then [`progress`], then [`quality`]. This uses enum-based dispatch
/// to provide a faster way to switch between actions `dyn trait`.
///
/// Each variant is documented with a link to the underlying action it executes.
///
/// [`buffs`]: crate::actions::buffs
/// [`misc`]: crate::actions::misc
/// [`progress`]: crate::actions::progress
/// [`quality`]: crate::actions::quality
#[ffxiv_action_enum]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum FfxivCraftingActions {
    /* Buff */
    /// [`InnerQuiet`]
    InnerQuiet,
    /// [`Veneration`]
    Veneration,
    /// [`WasteNot`]
    WasteNot,
    /// [`GreatStrides`]
    GreatStrides,
    /// [`Innovation`]
    Innovation,
    /// [`NameOfTheElements`]
    NameOfTheElements,
    /// [`FinalAppraisal`]
    FinalAppraisal,
    /// [`WasteNot2`]
    WasteNot2,
    /// [`Manipulation`]
    Manipulation,

    /* Misc */
    /// [`MastersMend`]
    MastersMend,
    /// [`Observe`]
    Observe,
    /// [`TricksOfTheTrade`]
    TricksOfTheTrade,
    /// [`DelicateSynthesis`]
    DelicateSynthesis,
    /// [`CarefulObservation`]
    CarefulObservation,

    /* Progress/Synthesis */
    /// [`BasicSynthesis`]
    BasicSynthesis,
    /// [`RapidSynthesis`]
    RapidSynthesis,
    /// [`BrandOfTheElements`]
    BrandOfTheElements,
    /// [`MuscleMemory`]
    MuscleMemory,
    /// [`CarefulSynthesis`]
    CarefulSynthesis,
    /// [`FocusedSynthesis`]
    FocusedSynthesis,
    /// [`Groundwork`]
    Groundwork,
    /// [`IntensiveSynthesis`]
    IntensiveSynthesis,

    /* Quality/Touch */
    /// [`BasicTouch`]
    BasicTouch,
    /// [`HastyTouch`]
    HastyTouch,
    /// [`StandardTouch`]
    StandardTouch,
    /// [`ByregotsBlessing`]
    ByregotsBlessing,
    /// [`PreciseTouch`]
    PreciseTouch,
    /// [`PatientTouch`]
    PatientTouch,
    /// [`PrudentTouch`]
    PrudentTouch,
    /// [`FocusedTouch`]
    FocusedTouch,
    /// [`Reflect`]
    Reflect,
    /// [`PreparatoryTouch`]
    PreparatoryTouch,
    /// [`TrainedEye`]
    TrainedEye,
}
