use std::marker::PhantomData;

use buffs::{progress::ProgressBuffs, quality::QualityBuffs};
use derivative::Derivative;

pub mod actions;
pub mod buffs;
pub mod conditions;
pub(crate) mod lookups;
pub mod quality_map;

use lookups::RecipeLevelRanges;
use quality_map::QualityMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CraftingSimulator<C, M>
where
    M: QualityMap,
{
    character: CharacterStats,
    recipe: RecipeStats,

    // Arguably these should be on `Recipe`, may move later
    // Expert recipes implied by Condition type
    conditions: C,
    // Whether we should map quality to HQ chance or collectibility,
    // and what tier of collectibility
    quality_map: PhantomData<M>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterStats {
    /* Character stats */
    craftsmanship: u16,
    control: u16,
    max_cp: u16,
    /// Actual level, 1..<max_char_lvl> (80 in Shb, 90 in EW etc)
    char_level: u8,
    /// Internal clvl; just a table lookup from char_level
    clvl: u16,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecipeStats {
    /* Recipe stats */
    // A lookup enum for rlvl stuff
    recipe_level: RecipeLevelRanges,

    max_durability: u16,
    max_quality: u32,
    max_progress: u32,

    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
}

#[derive(Clone, Copy, Derivative)]
#[derivative(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct CraftingState<'a, C, M>
where
    M: QualityMap,
{
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Debug = "ignore"
    )]
    recipe: &'a CraftingSimulator<C, M>,
    condition: C,

    curr_quality: u32,
    curr_progress: u32,
    curr_cp: u16,

    /* Quality-related */
    quality_buffs: QualityBuffs,

    /* Durability related */
    manipulation: u8,
    waste_not: u8,
    waste_not_2: u8,

    /* Progress Related */
    progress_buffs: ProgressBuffs,

    // Misc
    // Determines if muscle memory/reflection/trained eye is usable
    first_step: bool,
    // Allows for observation combo effects (if more combos get added we can abstract this, not worth it now)
    last_state_was_observation: bool,
}
