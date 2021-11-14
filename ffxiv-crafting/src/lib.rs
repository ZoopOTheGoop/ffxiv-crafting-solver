use std::marker::PhantomData;

use derivative::Derivative;

pub mod conditions;
pub(crate) mod lookups;
pub mod quality_map;

use quality_map::QualityMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CraftingSimulator<C, M>
where
    M: QualityMap,
{
    character: CharacterStats,
    recipe: RecipeStats,

    // Lookup table from clvl - rlvl
    lvl_mod: i16,

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
    max_cp: u8,
    /// Actual level, 1..<max_char_lvl> (80 in Shb, 90 in EW etc)
    char_level: u8,
    /// Internal clvl; just a table lookup from char_level
    clvl: u16,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecipeStats {
    /* Recipe stats */
    // Actual level, 1..<max_char_lvl> (80 in Shb, 90 in EW etc)
    recipe_level: u8,
    stars: u8,
    // Internal rlvl - recipe specific from lookup table
    rlvl: u16,
    rlvl_craftsmanship: u16,
    rlvl_control: u16,
    max_durability: u16,

    max_quality: u32,
    max_progress: u32,
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
    problem_state: &'a CraftingSimulator<C, M>,
    condition: C,

    curr_quality: u32,
    curr_progress: u32,
    curr_cp: u16,

    // Buffs; integers are stacks/steps left
    // There's an argument to be made for generalizing this more,
    // like a table or list of buffs that gets added and operated on
    // generically by actions acting on the state, but this is much easier
    // for storage and other concerns.

    /* Quality-related */
    inner_quiet: u8,
    innovation: u8,
    great_strides: u8,

    /* Durability related */
    manipulation: u8,
    waste_not: u8,
    waste_not_2: u8,

    /* Progress Related */
    // Since brand can't be used twice
    brand_of_elements: BrandState,
    veneration: u8,
    muscle_memory: u8,

    // Misc
    // Determines if muscle memory/reflection/trained eye is usable
    first_step: bool,
    last_state_was_observation: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Derivative)]
#[derivative(Default)]
enum BrandState {
    #[derivative(Default)]
    Unused,
    InProgress(u8),
    Used,
}
