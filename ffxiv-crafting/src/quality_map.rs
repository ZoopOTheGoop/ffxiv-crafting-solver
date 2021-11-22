use crate::lookups;

pub trait QualityMap {
    // HQ chance or Collectibility
    type Outcome: Sized;

    fn convert(quality: u32, recipe_quality: u32) -> Self::Outcome;
}

struct HQMap;

impl QualityMap for HQMap {
    type Outcome = HQChance;

    fn convert(quality: u32, recipe_quality: u32) -> HQChance {
        HQChance(lookups::lookup_hq(quality, recipe_quality))
    }
}

struct HQChance(u8);

impl From<NormalChance> for HQChance {
    fn from(other: NormalChance) -> Self {
        HQChance(100 - other.0)
    }
}
struct NormalChance(u8);

impl From<HQChance> for NormalChance {
    fn from(other: HQChance) -> Self {
        NormalChance(100 - other.0)
    }
}
