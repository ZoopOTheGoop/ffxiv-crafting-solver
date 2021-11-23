use std::marker::PhantomData;

use buffs::BuffState;
use conditions::Condition;
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
}

impl CharacterStats {
    const fn clvl(&self) -> u16 {
        lookups::CLVL[self.char_level as usize]
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecipeStats {
    /* Recipe stats */
    // A lookup enum for rlvl stuff
    recipe_level: RecipeLevelRanges,

    max_durability: u16,
    max_quality: u32,
    max_progress: u32,
}

#[derive(Clone, Copy, Derivative)]
#[derivative(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct CraftingState<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    recipe: &'a CraftingSimulator<C, M>,
    condition: C,

    curr_quality: u32,
    curr_progress: u32,
    curr_durability: i8,
    curr_cp: i16,

    buffs: BuffState,

    // Misc
    // Determines if muscle memory/reflection/trained eye is usable
    first_step: bool,
    // Allows for combo effects (if more combos get added we can abstract this, not worth it now)
    last_state_was_observation: bool,
    last_state_was_basic_touch: bool,
}

impl<'a, C, M> CraftingState<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    /// The base quality that any action operating on `quality` will modify with its `efficiency`.
    fn base_quality(&self) -> f64 {
        let iq = self
            .buffs
            .quality
            .inner_quiet
            .quality_mod(self.recipe.character.control);

        let rlvl = self.recipe.recipe.recipe_level;
        let clvl = self.recipe.character.clvl();

        let quality = iq * 35. / 100. + 35.;
        let quality = quality * (iq + 10_000.) / (rlvl.to_recipe_level_control() as f64 + 10_000.);
        quality * rlvl.to_quality_level_mod(clvl) as f64 / 100.
    }

    /// The base progress that any action operating on `progress` will modify with its `efficiency`.
    fn base_progress(&self) -> f64 {
        let craftsmanship = self.recipe.character.craftsmanship as f64;

        let rlvl = self.recipe.recipe.recipe_level;
        let clvl = self.recipe.character.clvl();

        let progress = craftsmanship * 21. / 100. + 2.;
        let progress = progress * (craftsmanship + 10_000.)
            / (rlvl.to_recipe_level_craftsmanship() as f64 / 10_000.);
        progress * rlvl.to_progress_level_mod(clvl) as f64 / 100.
    }
}
