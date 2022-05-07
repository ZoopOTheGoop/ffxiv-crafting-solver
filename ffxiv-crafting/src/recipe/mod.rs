//! Contains the definition for a crafting recipe. Originally it was just treated as a POD struct, but the complexity of dealing with RLVLs
//! made defining this a bit more involved.

use std::{
    error::Error,
    fmt::{self, Display},
    marker::PhantomData,
};

use crate::conditions::{
    tables::{ALL_EXPERT_CONDITIONS_UNUSED, EXPERT_CRAFT_1, EXPERT_CRAFT_2, NORMAL_CONDITIONS},
    Condition, ConditionBits,
};

mod tables;

/// A struct representing the `rlvl` or `RecipeLevel` value, a lookup that determines the difficulty of a craft.
#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct RLvl(pub u16);

impl Default for RLvl {
    fn default() -> Self {
        RLvl(1)
    }
}

impl RLvl {
    /// The minimum RLvl value. The internal tables are zero-indexed, but 0 contains a dummy value so this is 1
    pub const MIN_RLVL: u16 = 1;

    /// The maximum RLvl value. This is actually the last index, so if bounds checking, this should be used as `<=` rather than `<`)
    /// However, the maximum value may not necessarily be the highest level item at the given moment. The rlvl table is actually
    /// significantly padded with the last row repeated many, many times.
    pub const MAX_RLVL: u16 = tables::NUM_RLVLS as u16 - 1;

    /// Verifies your rlvl is within the allowed range (that is the range `[MIN_RLVL, MAX_RLVL]`).
    pub const fn verify_bounds(self) -> bool {
        self.0 >= Self::MIN_RLVL && self.0 <= Self::MAX_RLVL
    }

    /// Looks up the character level required to make recipes with this RLVL, 1-90 (as of EW)
    pub const fn character_level(self) -> u8 {
        tables::RECIPE_CLASS_LEVEL[self.0 as usize]
    }

    /// The progress divider used to make the recipe more difficult, this is essentially a flat penalty on
    /// craftsmanship for some harder recipes.
    pub const fn progress_divider(self) -> u16 {
        tables::PROGRESS_DIVIDER_RLVL[self.0 as usize]
    }

    /// The quality divider used to make the recipe more difficult, this is essentially a flat penalty on
    /// control for some harder recipes.
    pub const fn quality_divider(self) -> u16 {
        tables::QUALITY_DIVIDER_RLVL[self.0 as usize]
    }

    /// The progress modifier used to make the recipe more difficult, this is essentially a flat penalty on
    /// synthesis actions that is only applied on a current level (or harder) recipe.
    pub const fn progress_modifier(self) -> u16 {
        tables::PROGRESS_MODIFIER_RLVL[self.0 as usize]
    }

    /// The quality modifier used to make the recipe more difficult, this is essentially a flat penalty on
    /// touch actions that is only applied on a current level (or harder) recipe.
    pub const fn quality_modifier(self) -> u16 {
        tables::QUALITY_MODIFIER_RLVL[self.0 as usize]
    }

    /// The bitfield representing this recipe's allowed conditions.
    const fn condition_bits(self) -> ConditionBits {
        tables::CONDITIONS_RLVL[self.0 as usize]
    }

    /// The *base* target quality of a recipe with this rlvl, this is modified by a multiplier for the recipe itself and then divided by 100.
    pub const fn base_quality(self) -> u32 {
        tables::BASE_QUALITY[self.0 as usize]
    }

    /// The *base* target progress of a recipe with this rlvl, this is modified by a multiplier for the recipe itself and then divided by 100.
    ///
    /// Note: The actual entry in the table (both for the modifier and recipe level) is "difficulty". I suspect difficulty and progress became
    /// the same thing in Endwalker because there's a suspciously empty column near the quality and durability modifier columns in the recipe file,
    /// and I've seen references to the `RLVL_PROGRESS` before during ShB-times.
    pub const fn base_progress(self) -> u16 {
        tables::BASE_PROGRESS[self.0 as usize]
    }

    /// The *base* durability of a recipe with this rlvl, this is actually modified by a mulitiplier for the
    /// recipe itself. Fun fact - the only actual durabilities in the table are 60, 70, or 80, everything else (e.g. 40 or 35 durability
    /// intermediates) are modified by the recipe itself. This is multiplied as a float and rounded rather than truncated
    /// (you can check this because for some 60 durability recipes a modifier of 58 is used, which yields
    /// 34.8 but there are not 34 durability recipes - only 35).
    pub const fn base_durability(self) -> u16 {
        tables::BASE_DURABILITY[self.0 as usize]
    }

    /// The number of stars in the recipe descriptor, e.g. an 80** recipe would return 2.
    pub const fn recipe_stars(self) -> u8 {
        tables::RECIPE_STARS[self.0 as usize]
    }
}

/// Represents all the parameters of an FFXIV recipe needed for simulation. This looks up table values
/// upon construction and places them in the struct.
///
/// While all fields are marked `pub` for ease of access, modifying them manually is not recommended unless you're
/// trying to test weird hypotheticals wrt rlvl differences and strange durability values (or something), since all
/// these values are calculated off of internal lookup tables.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Recipe<C: Condition> {
    /// The RLvl that defines most of the properties of this recipe.
    rlvl: RLvl,

    /// The minimum level a crafter must be to make this recipe.
    pub required_character_level: u8,
    /// The number of stars on the recipe, e.g. 80**.
    pub stars: u8,

    /// The size of the quality bar.
    pub max_quality: u32,
    /// The size of the progress bar.
    pub max_progress: u32,
    /// The size of the durability bar.
    pub max_durability: i8,

    /// Pairs with `quality_modifier` in computation of the base effects
    /// of synthesis actions before ability modifiers are applied.
    pub quality_divider: u16,
    /// Pairs with `progress_divider` in computation of the base effects
    /// of synthesis actions before ability modifiers are applied.
    pub progress_divider: u16,

    /// Pairs with `quality_divider` in computation of the base effects
    /// of synthesis actions before ability modifiers are applied.
    pub quality_modifier: u16,
    /// Pairs with `progress_modifier` in computation of the base effects
    /// of synthesis actions before ability modifiers are applied.
    pub progress_modifier: u16,

    /// The recipe-specific modifier that determines this recipe's max durability. An integer-valued percentage.
    max_durability_mod: u16,
    /// The recipe-specific modifier that determines this recipe's max durability. An integer-valued percentage.
    max_quality_mod: u16,
    /// The recipe-specific modifier that determines this recipe's max durability. An integer-valued percentage.
    max_progress_mod: u16,

    _pd: PhantomData<C>,
}

impl<C: Condition> Recipe<C> {
    /// The underlying rlvl, this is not modifiable because it's needed for
    /// computing other values.
    pub fn rlvl(self) -> RLvl {
        self.rlvl
    }

    /// Recomputes the public struct values from the underlying `RLvl` and recipe-derived max
    /// value modifiers. These fields are hidden to prevent potential corruption. All these values
    /// are verified during construction, but are not re-verified here as they cannot change.
    pub fn compute_rlvl_values(&mut self) {
        self.required_character_level = self.rlvl.character_level();
        self.stars = self.rlvl.recipe_stars();

        // We need to verify this is right, durability has to be a rounding unless there's a weird order of operations
        // because truncation results in 34 durability for some recipes. Not sure about the rest, though.

        let durability_mod = self.max_durability_mod as f32 / 100.;
        let quality_mod = self.max_quality_mod as f32 / 100.;
        let progress_mod = self.max_progress as f32 / 100.;

        self.max_quality = (self.rlvl.base_quality() as f32 * quality_mod).round() as u32;
        self.max_durability = (self.rlvl.base_durability() as f32 * durability_mod).round() as i8;
        self.max_progress = (self.rlvl.base_progress() as f32 * progress_mod).round() as u32;

        self.quality_divider = self.rlvl.quality_divider();
        self.progress_divider = self.rlvl.progress_divider();

        self.quality_modifier = self.rlvl.quality_modifier();
        self.progress_modifier = self.rlvl.progress_modifier();
    }
}

impl<C: Condition + fmt::Debug + Default> Recipe<C> {
    /// Tries to crate an actual recipe object from the given parameters. Most values are pulled from the [RLvl] table,
    /// while the quality modifiers are divided by 100 and multiplied to determine recipe-specific deviations from the RLvl (such
    /// as modifying durability to 40 or 35).
    ///
    /// Note: the value named "progress_mod" is named for clarity, if you're converting from the recipe table the target value
    /// to extract is actually the *difficulty* modifier. I suspect difficulty and progress became
    /// the same thing in Endwalker because there's a suspciously empty column near the quality and
    /// durability modifier columns in the recipe file, and I've seen references to the `RLVL_PROGRESS` before during ShB-times.
    pub fn try_from_rlvl_modifiers(
        rlvl: RLvl,
        max_quality_mod: u16,
        max_progress_mod: u16,
        max_durability_mod: u16,
    ) -> Result<Self, RecipeError<C>> {
        if !rlvl.verify_bounds() {
            return Err(RecipeError::RLvlOutOfBounds(rlvl));
        }

        // Should we maybe make the bits an associated constant?
        // Might be cleaner. On the other hand that's leaking finnicky internal
        // data.
        if rlvl.condition_bits() != C::BITS {
            return Err(RecipeError::InvalidCondition {
                got: C::default(),
                expected: rlvl.condition_bits(),
            });
        }

        let mut me = Recipe {
            rlvl,
            max_quality_mod,
            max_progress_mod,
            max_durability_mod,

            required_character_level: 0,
            stars: 0,

            max_quality: 0,
            max_progress: 0,
            max_durability: 0,
            quality_divider: 0,
            progress_divider: 0,
            quality_modifier: 0,
            progress_modifier: 0,
            _pd: PhantomData {},
        };

        me.compute_rlvl_values();

        Ok(me)
    }
}

const fn bits_to_condition_error_msg(bits: ConditionBits) -> &'static str {
    match bits.0 {
        NORMAL_CONDITIONS => "the standard set of conditions (normal/good/poor/excellent); \
        use NoQARegularConditions or QARegularConditions",
        EXPERT_CRAFT_1 =>"the non-relic Ishgard Restoration expert conditions; use RestoExpertConditions",
        EXPERT_CRAFT_2 => "the Shadowbringers crafting relic expert conditions; use RelicExpertConditions",
        ALL_EXPERT_CONDITIONS_UNUSED => "an unused set of conditions; this isn't actually an in-use RLVL and no \
        condition is implemented that uses it. If you want to implement it yourself, it consists of all existing conditions EXCEPT \
        excellent/poor",
        // Want to use `unreachable` here, but it's not allowed in const functions atm
        _ => panic!("Got invalid condition in error that shouldn't even be in the table")
    }
}

/// The set of things that can go wrong when building a Recipe
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum RecipeError<C: Condition + fmt::Debug> {
    /// The RLvl was not a valid number
    RLvlOutOfBounds(RLvl),
    /// The Condition requested does not match the given RLvl
    #[allow(missing_docs)]
    InvalidCondition { got: C, expected: ConditionBits },
}

impl<C: Condition + fmt::Debug> Display for RecipeError<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RLvlOutOfBounds(RLvl(got)) => write!(
                f,
                "The given RLvl was out of the allowed bounds; \
            expected {} <= rlvl <= {} but got the value {}",
                RLvl::MIN_RLVL,
                got,
                RLvl::MAX_RLVL
            ),
            Self::InvalidCondition { got, expected } => {
                write!(
                    f,
                    "Attempted to construct recipe with condition {:?} but expected {}",
                    got,
                    bits_to_condition_error_msg(*expected)
                )
            }
        }
    }
}

impl<C: Condition + fmt::Debug> Error for RecipeError<C> {}
