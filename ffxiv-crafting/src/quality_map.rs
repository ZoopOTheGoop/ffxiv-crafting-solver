//! Contains types that map the crafting state's quality value to [`HQChance`]
//! or [`Collectability`].

mod tables {
    // It's 101 because it goes from [0-100], not [1-100]
    pub(crate) const HQ: [u8; 101] = [
        1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8,
        8, 8, 9, 9, 9, 10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16,
        16, 17, 17, 17, 18, 18, 18, 19, 19, 20, 20, 21, 22, 23, 24, 26, 28, 31, 34, 38, 42, 47, 52,
        58, 64, 68, 71, 74, 76, 78, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 94, 96, 98,
        100,
    ];

    pub(super) const fn lookup_hq(quality: u32, recipe_quality: u32) -> u8 {
        // Compute integer percentage without casting -- this gives the same result as going to
        // float and then truncating from conversion
        let raw_chance = (quality * 200 + recipe_quality) / (recipe_quality * 2);

        // Can't use `min` in const functions :(
        HQ[if raw_chance > 100 { 100 } else { raw_chance } as usize]
    }
}

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
        HQChance(tables::lookup_hq(
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
