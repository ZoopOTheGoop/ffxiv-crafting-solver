//! Contains types that map the crafting state's quality value to [`HQChance`]
//! or [`Collectability`].

use crate::lookups;

/// Maps the `quality` property to either [`HQChance`] or
/// [`Collectability`], depending on the craft at hand.
pub trait QualityMap: Copy {
    /// HQ/Collectability
    type Outcome: Sized;

    /// Converts the quality value to the given outcome using
    /// FFXIV's own logic, (for HQ this is a lookup table).
    fn convert(quality: u32, recipe_quality: u32) -> Self::Outcome;
}

/// Maps quality to [`HQChance`].
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct HQMap;

impl QualityMap for HQMap {
    type Outcome = HQChance;

    fn convert(quality: u32, recipe_quality: u32) -> HQChance {
        HQChance(lookups::lookup_hq(
            quality.min(recipe_quality),
            recipe_quality,
        ))
    }
}

/// The chance from 1-100 that an item will come out HQ at the current
/// quality. This can convert into its dual, [`NQChance`].
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HQChance(pub u8);

impl From<NQChance> for HQChance {
    fn from(other: NQChance) -> Self {
        HQChance(100 - other.0)
    }
}

impl Default for HQChance {
    fn default() -> Self {
        HQChance(1)
    }
}

/// The chance from 1-100 that an item will come out NQ at the current
/// quality. This can convert into its dual, [`HQChance`].
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NQChance(pub u8);

impl Default for NQChance {
    fn default() -> Self {
        NQChance(99)
    }
}

impl From<HQChance> for NQChance {
    fn from(other: HQChance) -> Self {
        NQChance(100 - other.0)
    }
}

/// Maps quality to the proper collectability rating, this is actually
/// just division by 10.
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollectabilityMap;

impl QualityMap for CollectabilityMap {
    type Outcome = Collectability;

    fn convert(quality: u32, recipe_quality: u32) -> Self::Outcome {
        Collectability(quality.min(recipe_quality) / 10)
    }
}

/// The collectability of an item, for turnins. The tiers
/// are recipe (or at least rlvl) specific and should be mapped by the user.
pub struct Collectability(pub u32);
