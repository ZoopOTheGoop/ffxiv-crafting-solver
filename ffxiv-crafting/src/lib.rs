#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::marker::PhantomData;

use actions::StateDelta;
use buffs::BuffState;
use conditions::Condition;
use derivative::Derivative;

pub mod actions;
pub mod buffs;
pub mod conditions;
pub(crate) mod lookups;
pub mod quality_map;

#[doc(inline)]
pub use lookups::RecipeLevelRanges;
use quality_map::QualityMap;
use rand::Rng;

/// The overall simulator problem. This is actually just the definition that gives
/// structure to the problem, such as the recipe used and character stats. It's mostly just
/// a plain old data structure, but some of its members can compute useful state such as
/// [`RecipeStats`] computing `rlvl` lookups.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CraftingSimulator<C, M>
where
    M: QualityMap,
{
    /// Stats of the character making this recipe.
    pub character: CharacterStats,

    /// The stats of the Recipe - *after* applying things like
    /// internal modifiers.
    pub recipe: RecipeStats,

    /// The conditions this recipe can take on - essentially either
    /// the typical Normal/Good/Excellent/Poor distribution or else
    /// one of the expert recipes.
    pub conditions: C,

    quality_map: PhantomData<M>,
}

/// The stats of the a FFXIV character - these are *after* any buffs
/// or food. It can look up `clvl` based on your character level.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterStats {
    /* Character stats */
    #[allow(missing_docs)]
    pub craftsmanship: u16,
    #[allow(missing_docs)]
    pub control: u16,
    #[allow(missing_docs)]
    pub max_cp: i16,

    /// Actual level, 1..<max_char_lvl> (80 in Shb, 90 in EW etc)
    pub char_level: u8,
}

impl CharacterStats {
    /// Looks up the character's [`clvl`] in the proper reference table.
    ///
    /// This is entirely based on actual character level.
    const fn clvl(&self) -> u16 {
        lookups::CLVL[self.char_level as usize]
    }
}

/// The stats of a recipe, containing both its level as well as the
/// three states that govern a recipe's status.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecipeStats {
    /// The recipe "level" taking into account level cap recipes
    /// and stars, as well as the subtle distinctions between them.
    ///
    /// Can look up a great number of things related to the internal
    /// `rlvl`.
    pub recipe_level: RecipeLevelRanges,

    /// The durability this recipe starts at and cannot go above.
    max_durability: i8,

    /// The maximum quality of the recipe for determining HQ/collectability.
    max_quality: u32,

    /// The maximum progress of a recipe, when the state hits this value
    /// the recipe is completed.
    max_progress: u32,
}

/// The current state of the crafting simulation. The vast majority of types
/// operate on this. Note that this is still a bit "low level" and doesn't track some
/// convenience options such as number of actions taken. Solvers will likely have to
/// wrap this.
#[derive(Clone, Copy, Derivative)]
#[derivative(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct CraftingState<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    /// The problem definition -- this doesn't change over the
    /// course of the simulation, so it's essentially an immutable
    /// thing to check against.
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    pub problem_def: &'a CraftingSimulator<C, M>,

    /// The current [`Condition`] based on the type of recipe this is,
    /// has effects on the potency of actions.
    pub condition: C,

    /// The current quality value, this can go over the actual maximum, but will
    /// be clamped when computing the output. This behavior is useful if you want to see
    /// how much an action "overperformed".
    pub curr_quality: u32,

    /// The current quality value, this can go over the actual maximum.
    /// This behavior is useful if you want to see how much an action "overperformed".
    pub curr_progress: u32,

    /// The current durability value, this can go under 0.
    /// This behavior is useful if you want to see how much grace you may have in choosing
    /// other actions.
    pub curr_durability: i8,

    /// The character's current CP resource. This can go under 0.
    /// This behavior is useful if you want to see how much grace you may have in choosing
    /// other actions.
    pub curr_cp: i16,

    /// The current state of any buffs present in the simulation. This includes some
    /// fake "buffs" used to trigger combo actions.
    pub buffs: BuffState,

    /// Determines if several first turn-only actions are usable. Technically
    /// FFXIV itself uses "step count", but that makes state ranking harder to
    /// reason about. Solver implementors may want to track that themselves.
    first_step: bool,
}

impl<'a, C, M> CraftingState<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    /// The base quality that any action operating on `quality` will modify with its `efficiency`.
    pub fn base_quality(&self) -> f64 {
        let control = self.problem_def.character.control as f64;

        let rlvl = self.problem_def.recipe.recipe_level;
        let clvl = self.problem_def.character.clvl();

        let quality = control * 35. / 100. + 35.;
        let quality =
            quality * (control + 10_000.) / (rlvl.to_recipe_level_control() as f64 + 10_000.);
        quality * rlvl.to_quality_level_mod(clvl) as f64 / 100.
    }

    /// The base progress that any action operating on `progress` will modify with its `efficiency`.
    pub fn base_progress(&self) -> f64 {
        let craftsmanship = self.problem_def.character.craftsmanship as f64;

        let rlvl = self.problem_def.recipe.recipe_level;
        let clvl = self.problem_def.character.clvl();

        let progress = craftsmanship * 21. / 100. + 2.;
        let progress = progress * (craftsmanship + 10_000.)
            / (rlvl.to_recipe_level_craftsmanship() as f64 / 10_000.);
        progress * rlvl.to_progress_level_mod(clvl) as f64 / 100.
    }

    /// Generates the next state from the given delta, including sampling the new condition.
    pub fn gen_succ<R: Rng>(self, delta: StateDelta, rng: &mut R) -> Self {
        Self {
            condition: self.condition.sample(rng),
            ..self + delta
        }
    }
}
