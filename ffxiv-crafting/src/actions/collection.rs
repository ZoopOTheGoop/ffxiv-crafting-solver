//! Provides collections containing all the actions for fast dispatch in simulators. These are shims that just defer to
//! the underlying implementation with as few branches as possible.

use super::{buffs::*, misc::*, progress::*, quality::*};

use ffxiv_crafting_derive::PassthroughAction;

/// "A collection of all actions in FFXIV. They're organized by [`buffs`], then
/// [`misc`], then [`progress`], then [`quality`]. This uses enum-based dispatch
/// to provide a faster way to switch between actions `dyn trait`.
///
/// Each variant is documented with a link to the underlying action it executes.
///
/// This uses some truly ugly autogen magic to prevent having to copy+paste probably
/// several thousand lines of enum variant matching. Its associated failure is
/// [`ComboFailure`], which simply defers to the underlying action's failure type
/// (this is manually implemented in the autogen).
///
/// If you want to see *why* I didn't manually implement this you can get an
/// *extremely* abridged look by looking over at [`ComboFailure`]'s manual
/// implementation and then imagine having to do something like that for
/// every variant of this enum, for every function of every trait.
///
/// [`buffs`]: crate::actions::buffs
/// [`misc`]: crate::actions::misc
/// [`progress`]: crate::actions::progress
/// [`quality`]: crate::actions::quality
/// [`ComboFailure`]: crate::actions::failure::ComboFailure
#[derive(PassthroughAction)]
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
    /// [`PrudentSynthesis`]
    PrudentSynthesis,

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
    /// [`AdvancedTouch`]
    AdvancedTouch,
}
