use ffxiv_crafting_derive::Condition;
use rand::distributions::Distribution;

use crate::lookups::{
    CpUsageModifier, DurabilityModifier, ProgressModifier, QualityModifier, StatusDurationModifier,
    SuccessRateModifier,
};

// // Two different types is sad, if we get the #[exhaustive_patterns] feature though
// // we can create an unreachable variant that holds a PhantomData to QA/NoQA that defines
// // the distribution

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition)]
pub enum NoQARegularConditions {
    Normal,
    #[ffxiv(quality)]
    Good,
    #[ffxiv(quality)]
    Excellent,
    #[ffxiv(quality)]
    Poor,
}

impl Distribution<Self> for NoQARegularConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        match self {
            Self::Good | Self::Poor => Self::Normal,
            Self::Excellent => Self::Poor,
            Self::Normal => {
                let roll: u8 = rng.gen_range(0..100);
                if roll < 4 {
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition)]
pub enum QARegularConditions {
    Normal,
    #[ffxiv(quality)]
    Good,
    #[ffxiv(quality)]
    Excellent,
    #[ffxiv(quality)]
    Poor,
}

impl Distribution<Self> for QARegularConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        match self {
            Self::Good | Self::Poor => Self::Normal,
            Self::Excellent => Self::Poor,
            Self::Normal => {
                let roll: u8 = rng.gen_range(0..100);
                if roll < 4 + 5 {
                    Self::Good
                } else if roll < 20 + 4 + 5 {
                    Self::Excellent
                } else {
                    Self::Normal
                }
            }
        }
    }
}

// Corresponds to EXPERT_CRAFT_1
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition)]
pub enum RestoExpertConditions {
    Normal,
    #[ffxiv(quality)]
    Good,
    #[ffxiv(success)]
    Centered,
    #[ffxiv(cp)]
    Pliant,
    #[ffxiv(durability)]
    Sturdy,
}

impl Distribution<Self> for RestoExpertConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        const WEIGHTS: [u8; 4] = [12, 15, 12, 15];
        const RESULTS: [RestoExpertConditions; 4] = [
            RestoExpertConditions::Good,
            RestoExpertConditions::Centered,
            RestoExpertConditions::Pliant,
            RestoExpertConditions::Sturdy,
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

// Corresponds to EXPERT_CRAFT_2
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Condition)]
pub enum RelicExpertConditions {
    Normal,
    #[ffxiv(quality)]
    Good,
    #[ffxiv(cp)]
    Pliant,
    #[ffxiv(durability)]
    Sturdy,
    #[ffxiv(progress)]
    Malleable,
    #[ffxiv(status)]
    Primed,
}

impl Distribution<Self> for RelicExpertConditions {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        const WEIGHTS: [u8; 4] = [12, 12, 12, 12];
        const RESULTS: [RelicExpertConditions; 4] = [
            RelicExpertConditions::Good,
            RelicExpertConditions::Pliant,
            RelicExpertConditions::Malleable,
            RelicExpertConditions::Primed,
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

pub trait Condition: Copy + Sized + Distribution<Self> {
    fn to_quality_modifier(self) -> QualityModifier;
    fn to_progress_modifier(self) -> ProgressModifier;
    fn to_success_rate_modifier(self) -> SuccessRateModifier;
    fn to_durability_modifier(self) -> DurabilityModifier;
    fn to_status_duration_modifier(self) -> StatusDurationModifier;
    fn to_cp_usage_modifier(self) -> CpUsageModifier;
}