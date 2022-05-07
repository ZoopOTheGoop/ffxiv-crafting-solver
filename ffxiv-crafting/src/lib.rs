#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::marker::PhantomData;

use actions::{Action, ActionOutcome, RandomAction, StateDelta};
use buffs::BuffState;
use conditions::Condition;
use derivative::Derivative;

pub mod actions;
pub mod buffs;
pub mod conditions;
pub mod quality_map;
pub mod recipe;

use quality_map::QualityMap;
use rand::Rng;

use crate::recipe::Recipe;

mod tables {
    /// This is the entire CLVL table which apparently goes all the way to level 201, as with most tables this has
    /// a dummy level 0 as well
    pub(crate) const CLVL: [u16; 202] = [
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
        63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
        86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
        107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
        125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
        143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
        161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178,
        179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196,
        197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214,
        215, 216, 217, 218,
    ];
}

/// The overall simulator problem. This is actually just the definition that gives
/// structure to the problem, such as the recipe used and character stats. It's mostly just
/// a plain old data structure, but some of its members can compute useful state such as
/// [`RecipeStats`] computing `rlvl` lookups.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CraftingSimulator<C, M>
where
    M: QualityMap,
    C: Condition + Copy,
{
    /// Stats of the character making this recipe.
    pub character: CharacterStats,

    /// The stats of the Recipe - *after* applying things like
    /// internal modifiers.
    pub recipe: Recipe<C>,

    quality_map: PhantomData<M>,
}

impl<C, M> CraftingSimulator<C, M>
where
    C: Condition + Copy,
    M: QualityMap,
{
    /// Builds a new simulation for a given character making a given recipe.
    pub fn from_character_recipe(character: CharacterStats, recipe: Recipe<C>) -> Self {
        CraftingSimulator {
            character,
            recipe,
            quality_map: PhantomData {},
        }
    }
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

    /// Actual level, 1..90 in Endwalker
    pub char_level: u8,
}

impl CharacterStats {
    /// Looks up the character's [`clvl`] in the proper reference table.
    ///
    /// This is entirely based on actual character level.
    const fn clvl(&self) -> u16 {
        #[cfg(debug_assertions)]
        if self.char_level < 1 || self.char_level > 90 {
            panic!("While the table goes higher, this is out of bounds for EW so we're not allowing it");
        }
        tables::CLVL[self.char_level as usize]
    }
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

        let quality = (control * 10.) / (self.problem_def.recipe.quality_divider as f64) + 35.;
        if self.problem_def.character.clvl() <= self.problem_def.recipe.rlvl().0 {
            quality * self.problem_def.recipe.quality_modifier as f64 * 0.01
        } else {
            quality
        }
    }

    /// The base progress that any action operating on `progress` will modify with its `efficiency`.
    pub fn base_progress(&self) -> f64 {
        let craftsmanship = self.problem_def.character.craftsmanship as f64;

        let progress =
            (craftsmanship * 10.) / (self.problem_def.recipe.progress_divider as f64) + 2.;
        if self.problem_def.character.clvl() <= self.problem_def.recipe.rlvl().0 {
            progress * self.problem_def.recipe.progress_modifier as f64 * 0.01
        } else {
            progress
        }
    }

    /// Generates the next state from the given delta, including sampling the new condition.
    pub fn gen_succ<R: Rng>(self, delta: StateDelta, condition_rng: &mut R) -> Self {
        Self {
            condition: if delta.time_passed {
                self.condition.sample(condition_rng)
            } else {
                self.condition
            },
            ..self + delta
        }
    }

    /// Performs an action on the current state, yielding an [`Outcome`] value that corresponds to
    /// the [`ActionOutcome`]. This uses [`act_random`] under the hood, and thus will panic if the action
    /// cannot execute due to being out of CP or an action being used in an illegal state.
    ///
    /// [`Action`]s and [`Condition`]s use different [`Rng`]s for maximum reproducibility, despite
    /// the minor inconvenience of managing two.
    ///
    /// The [`Condition`] is only rerolled if the [`Outcome`] is [`InProgress`], otherwise the generated successor
    /// state inherits the [`Condition`] of this state.
    ///
    /// [`act_random`]: crate::actions::Action::act_random
    /// [`InProgress`]: Outcome::InProgress
    pub fn act<A: Action + RandomAction, R1: Rng, R2: Rng>(
        self,
        action: A,
        action_rng: &mut R1,
        condition_rng: &mut R2,
    ) -> Outcome<'a, C, M> {
        let outcome = action.act_random(action_rng, &self).unwrap();

        match outcome {
            ActionOutcome::Completed(delta) => Outcome::Completed {
                state: self + delta,
                delta,
                outcome: outcome.map_quality(&self).unwrap(),
            },
            ActionOutcome::Failure(delta) => Outcome::Failure {
                state: self + delta,
                delta,
            },
            ActionOutcome::InProgress(delta) => Outcome::InProgress {
                state: self.gen_succ(delta, condition_rng),
                delta,
            },
        }
    }
}

/// The outcome of executing an [`Action`] on a given [`CraftingState`]. This is analogous to
/// [`ActionOutcome`]. In each of the variants, `state` is the next state, and `delta`
/// is the [`StateDelta`] which was applied to the previous state to create it.
pub enum Outcome<'a, C, M>
where
    C: Condition,
    M: QualityMap,
{
    /// This crafting state has neither failed nor completed (i.e. its durability has not hit zero,
    /// and its progress bar is not full).
    InProgress {
        /// The next state in the chain.
        state: CraftingState<'a, C, M>,

        /// The delta used to create the state.
        delta: StateDelta,
    },

    /// This crafting state has successfully completed, i.e. its prograss bar is full. This contains
    /// an `outcome` denoting either its HQ Chance from 1-100 or its Collectability.
    Completed {
        /// The final state. Note that progress may > than the actual progress, and durability may be below zero.
        /// This allows analyzers to see how much "slack" that actions might have.
        state: CraftingState<'a, C, M>,

        /// The delta used to create the state.
        delta: StateDelta,

        /// The HQ chance of the item, or its collectability.
        outcome: M::Outcome,
    },

    /// This crafting state failed, i.e. its durability hit zero without its progress bar becoming full.
    Failure {
        /// The state encoding the failure.
        ///
        /// Note that durability may be below zero instead of zero, to allow analyzers to see how much "slack"
        /// it may have.
        state: CraftingState<'a, C, M>,

        /// The delta used to create the state.
        delta: StateDelta,
    },
}
