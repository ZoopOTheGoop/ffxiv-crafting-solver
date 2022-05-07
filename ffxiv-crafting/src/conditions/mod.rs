//! Implements the [`Condition`] trait, which determines the distribution of conditions
//! in the crafting state, providing various benefits or detriments to the crafter depending
//! on the actions used, as well as enabling certain [`actions`].
//!
//! [`actions`]: crate::actions

use std::error::Error;

use derivative::Derivative;
use ffxiv_crafting_derive::Condition;
use rand::distributions::Distribution;

pub(crate) mod tables;

use tables::{
    CpUsageModifier, DurabilityModifier, ProgressModifier, QualityModifier, StatusDurationModifier,
    SuccessRateModifier,
};

/// The raw bits that make up a condition, largely used internally but needed for some info so it's exposed here.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionBits(pub u16);

pub mod raw_conditions {
    //! The raw condition modifiers used by [`Condition`]
    //!
    //! [`Condition`]: super::Condition

    #[doc(inline)]
    pub use super::tables::{
        CpUsageModifier, DurabilityModifier, ProgressModifier, QualityModifier,
        StatusDurationModifier, SuccessRateModifier,
    };
}

// // Two different types is sad, if we get the #[exhaustive_patterns] feature though
// // we can create an unreachable variant that holds a PhantomData to QA/NoQA that defines
// // the distribution

/// Defines the typical set of conditions for the vast majority of FFXIV crafting recipes.
///
/// There are two variants of this struct: this one and [`QARegularConditions`]. This one encodes
/// the state distribution for levels before you get the +5% bonus to the [`Good`] condition appearing
/// after a normal, and the one one encodes after. These two will likely be merged once
/// [`exhaustive_patterns`] stabilizes, in favor of an unconstructable variant holding [`PhantomData`]
/// encoding whether the player has gotten this.
///
/// [`Good`]: NoQARegularConditions::Good
/// [`exhaustive_patterns`]: https://github.com/rust-lang/rust/issues/51085
/// [`PhantomData`]: std::marker::PhantomData
#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition, Derivative
)]
#[derivative(Default)]
#[ffxiv(condition(bits = "NORMAL_CONDITIONS"))]
pub enum NoQARegularConditions {
    /// Normal condition -- nothing special.
    #[derivative(Default)]
    Normal,

    /// 20% chance to occur, provides a 50% efficiency bonus
    /// to quality and enables some actions. [`Normal`] must occur
    /// after this.
    ///
    /// [`Normal`]: NoQARegularConditions::Normal
    #[ffxiv(affects = "quality")]
    Good,

    /// 4% chance to occur, provides a 4x efficiency bonus
    /// to quality and enables some actions. [`Poor`] must occur
    /// after this
    ///
    /// [`Poor`]: NoQARegularConditions::Poor
    #[ffxiv(affects = "quality")]
    Excellent,

    /// Always occurs after [`Excellent`]. Provides a 50%
    /// malus to efficiency. [`Normal`] must occur
    /// after this.
    ///
    /// [`Excellent`]: NoQARegularConditions::Excellent
    /// [`Normal`]: NoQARegularConditions::Normal
    #[ffxiv(affects = "quality")]
    Poor,
}

impl Distribution<Self> for NoQARegularConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        match self {
            Self::Good | Self::Poor => Self::Normal,
            Self::Excellent => Self::Poor,
            Self::Normal => {
                let roll: u8 = rng.gen_range(0..100);
                if roll < 20 {
                    Self::Good
                } else if roll < 20 + 4 {
                    Self::Excellent
                } else {
                    Self::Normal
                }
            }
        }
    }
}

impl TryFrom<ConditionBits> for NoQARegularConditions {
    type Error = Box<dyn Error>;

    fn try_from(value: ConditionBits) -> Result<Self, Self::Error> {
        if value.0 == tables::NORMAL_CONDITIONS {
            Ok(Self::default())
        } else {
            Err("Bits don't match this condition pattern".into())
        }
    }
}

/// Defines the typical set of conditions for the vast majority of FFXIV crafting recipes.
///
/// There are two variants of this struct: this one and [`NoQARegularConditions`]. This one encodes
/// the state distribution for levels after you get the +5% bonus to the [`Good`] condition appearing
/// after a normal, and the one one encodes after. These two will likely be merged once
/// [`exhaustive_patterns`] stabilizes, in favor of an unconstructable variant holding [`PhantomData`]
/// encoding whether the player has gotten this.
///
/// [`Good`]: QARegularConditions::Good
/// [`exhaustive_patterns`]: https://github.com/rust-lang/rust/issues/51085
/// [`PhantomData`]: std::marker::PhantomData
#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition, Derivative
)]
#[derivative(Default)]
#[ffxiv(condition(bits = "NORMAL_CONDITIONS"))]
pub enum QARegularConditions {
    /// Normal condition -- nothing special.
    #[derivative(Default)]
    Normal,

    /// 25% chance to occur, provides a 50% efficiency bonus
    /// to quality and enables some actions. [`Normal`] must occur
    /// after this.
    ///
    /// [`Normal`]: QARegularConditions::Normal
    #[ffxiv(affects = "quality")]
    Good,

    /// 4% chance to occur, provides a 4x efficiency bonus
    /// to quality and enables some actions. [`Poor`] must occur
    /// after this
    ///
    /// [`Poor`]: QARegularConditions::Poor
    #[ffxiv(affects = "quality")]
    Excellent,

    /// Always occurs after [`Excellent`]. Provides a 50%
    /// malus to efficiency. [`Normal`] must occur
    /// after this.
    ///
    /// [`Excellent`]: QARegularConditions::Excellent
    /// [`Normal`]: QARegularConditions::Normal
    #[ffxiv(affects = "quality")]
    Poor,
}

impl Distribution<Self> for QARegularConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        match self {
            Self::Good | Self::Poor => Self::Normal,
            Self::Excellent => Self::Poor,
            Self::Normal => {
                let roll: u8 = rng.gen_range(0..100);
                if roll < 20 + 5 {
                    Self::Good
                } else if roll < 20 + 5 + 4 {
                    Self::Excellent
                } else {
                    Self::Normal
                }
            }
        }
    }
}

impl TryFrom<ConditionBits> for QARegularConditions {
    type Error = Box<dyn Error>;

    fn try_from(value: ConditionBits) -> Result<Self, Self::Error> {
        if value.0 == tables::NORMAL_CONDITIONS {
            Ok(Self::default())
        } else {
            Err("Bits don't match this condition pattern".into())
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ConditionBits> for QARegularConditions {
    fn into(self) -> ConditionBits {
        ConditionBits(tables::NORMAL_CONDITIONS)
    }
}

// Corresponds to EXPERT_CRAFT_1

/// The set of conditions used in expert crafting for oddly delicate materials for ShB relics.
///
/// Note: there are a couple of questions on the distribution I need to iron out. Specifically if
/// [`Good`] still forces [`Normal`] after. The other conditions seems to be completely independently
/// distributed (i.e. you can roll the same one twice in a row, and roll another condition after
/// without going through [`Normal`] first), but it's possible [`Good`] still acts the normal way.
///
/// I'm also unsure if the 12% rate on [`Good`] is affected by Quality Assurance like normal crafting,
/// making it 17%. These are minor details, but we want to be sure we have it right at some point.
///
/// [`Good`]: RelicExpertConditions::Good
/// [`Normal`]: RelicExpertConditions::Normal
#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition, Derivative
)]
#[derivative(Default)]
#[ffxiv(condition(expert, bits = "EXPERT_CRAFT_1"))]
pub enum RelicExpertConditions {
    /// Normal condition -- nothing special.
    #[derivative(Default)]
    Normal,

    /// 12% chance to occur, provides a 50% efficiency bonus
    /// to quality and enables some actions. Not sure if
    /// [`Normal`] must occur after this, but currently assuming
    /// "no".
    ///
    /// [`Normal`]: RelicExpertConditions::Normal
    #[ffxiv(affects = "quality")]
    Good,

    /// 15% chance to occur, provides a 25% success rate
    /// boost to actions taken.
    #[ffxiv(affects = "success")]
    Centered,

    /// 12% chance to occur, causes actions to use half CP.
    #[ffxiv(affects = "cp")]
    Pliant,

    /// 15% chance to occur, causes durability usage to be halved.
    #[ffxiv(affects = "durability")]
    Sturdy,
}

impl Distribution<Self> for RelicExpertConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        const WEIGHTS: [u8; 4] = [12, 15, 12, 15];
        const RESULTS: [RelicExpertConditions; 4] = [
            RelicExpertConditions::Good,
            RelicExpertConditions::Centered,
            RelicExpertConditions::Pliant,
            RelicExpertConditions::Sturdy,
        ];
        let roll: u8 = rng.gen_range(0..100);

        let mut acc = 0;
        for (weight, result) in WEIGHTS.iter().zip(RESULTS) {
            acc += weight;
            if roll < acc {
                return result;
            }
        }

        Self::Normal
    }
}

impl TryFrom<ConditionBits> for RelicExpertConditions {
    type Error = Box<dyn Error>;

    fn try_from(value: ConditionBits) -> Result<Self, Self::Error> {
        if value.0 == tables::EXPERT_CRAFT_1 {
            Ok(Self::default())
        } else {
            Err("Bits don't match this condition pattern".into())
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ConditionBits> for RelicExpertConditions {
    fn into(self) -> ConditionBits {
        ConditionBits(tables::EXPERT_CRAFT_1)
    }
}

/// The set of conditions used in expert crafting for most Ishgard Resto Expert Crafts.
///
/// Note: there are a couple of questions on the distribution I need to iron out. Specifically if
/// [`Good`] still forces [`Normal`] after. The other conditions seems to be completely independently
/// distributed (i.e. you can roll the same one twice in a row, and roll another condition after
/// without going through [`Normal`] first), but it's possible [`Good`] still acts the normal way.
///
/// I'm also unsure if the 12% rate on [`Good`] is affected by Quality Assurance like normal crafting,
/// making it 17%. These are minor details, but we want to be sure we have it right at some point.
///
/// [`Good`]: RestoExpertConditions::Good
/// [`Normal`]: RestoExpertConditions::Normal
#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition, Derivative
)]
#[derivative(Default)]
#[ffxiv(condition(expert, bits = "EXPERT_CRAFT_2"))]
pub enum RestoExpertConditions {
    /// Normal condition -- nothing special.
    #[derivative(Default)]
    Normal,

    /// 12% chance to occur, provides a 50% efficiency bonus
    /// to quality and enables some actions. Not sure if
    /// [`Normal`] must occur after this, but currently assuming
    /// "no".
    ///
    /// [`Normal`]: RestoExpertConditions::Normal
    #[ffxiv(affects = "quality")]
    Good,

    /// 12% chance to occur, causes actions to use half CP.
    #[ffxiv(affects = "cp")]
    Pliant,

    /// 15% chance to occur, causes durability usage to be halved.
    #[ffxiv(affects = "durability")]
    Sturdy,

    /// 12% chance to occur, gives a 50% efficiency boost to progress.
    ///
    /// This is essentially like a [`Good`] for [synthesis] actions,
    /// but without enabling some abilities like [`IntensiveSynthesis`]
    /// or [`TricksOfTheTrade`].
    ///
    /// [`Good`]: RestoExpertConditions::Good
    /// [synthesis]: crate::actions::progress
    /// [`IntensiveSynthesis`]: crate::actions::progress::IntensiveSynthesis
    /// [`TricksOfTheTrade`]: crate::actions::misc::TricksOfTheTrade
    #[ffxiv(affects = "progress")]
    Malleable,

    /// 12% chance to occur. Causes [`DurationalBuff`]s to gain two
    /// extra free ticks.
    ///
    /// [`DurationalBuff`]: crate::buffs::DurationalBuff
    #[ffxiv(affects = "status")]
    Primed,
}

impl Distribution<Self> for RestoExpertConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        const WEIGHTS: [u8; 4] = [12, 12, 12, 12];
        const RESULTS: [RestoExpertConditions; 4] = [
            RestoExpertConditions::Good,
            RestoExpertConditions::Pliant,
            RestoExpertConditions::Malleable,
            RestoExpertConditions::Primed,
        ];
        let roll: u8 = rng.gen_range(0..100);

        let mut acc = 0;
        for (weight, result) in WEIGHTS.iter().zip(RESULTS) {
            acc += weight;
            if roll < acc {
                return result;
            }
        }

        Self::Normal
    }
}

impl TryFrom<ConditionBits> for RestoExpertConditions {
    type Error = Box<dyn Error>;

    fn try_from(value: ConditionBits) -> Result<Self, Self::Error> {
        if value.0 == tables::EXPERT_CRAFT_2 {
            Ok(Self::default())
        } else {
            Err("Bits don't match this condition pattern".into())
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ConditionBits> for RestoExpertConditions {
    fn into(self) -> ConditionBits {
        ConditionBits(tables::EXPERT_CRAFT_2)
    }
}

/// The general trait all conditions implement, this just maps the conditions
/// to their modifiers in the internal condition tables. This is autoderived with
/// a proc macro and most of the functions are self explanatory. The modifiers are all
/// numbered enum variants corresponding to the proper tag.
///
/// Note that things are "smooshed" into the variant `Normal` if they're not one of the
/// variants that affect the given property.
pub trait Condition: Copy + Sized + Distribution<Self> {
    #![allow(missing_docs)]

    const EXPERT: bool = false;
    const BITS: ConditionBits;

    fn to_quality_modifier(self) -> QualityModifier;
    fn to_progress_modifier(self) -> ProgressModifier;
    fn to_success_rate_modifier(self) -> SuccessRateModifier;
    fn to_durability_modifier(self) -> DurabilityModifier;
    fn to_status_duration_modifier(self) -> StatusDurationModifier;
    fn to_cp_usage_modifier(self) -> CpUsageModifier;
    fn is_good(self) -> bool;
    fn is_excellent(self) -> bool {
        false
    }
}
