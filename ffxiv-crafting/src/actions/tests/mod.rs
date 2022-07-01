#![allow(missing_docs)]

mod buffs;

use lazy_static::lazy_static;
use rand::RngCore;

use crate::{
    conditions::QARegularConditions,
    quality_map::HQMap,
    recipe::{RLvl, Recipe},
    CharacterStats, CraftingSimulator,
};

/// My character's stats with melded Pactmaker under Cunning Craftsman's Syrup and Tsai you Vounou in 6.1
const TEST_STATS: CharacterStats = CharacterStats {
    craftsmanship: 3691,
    control: 3664,
    max_cp: 564,
    char_level: 90,
};

lazy_static! {
    /// This is for any Classical gear as of 6.1
    static ref TEST_RECIPE_CLASSICAL: Recipe<QARegularConditions> = Recipe::<QARegularConditions>::try_from_rlvl_modifiers(RLvl(580), 140, 100, 100).unwrap();
    static ref CLASSICAL_SIMULATOR: CraftingSimulator<QARegularConditions, HQMap> = CraftingSimulator::from_character_recipe(TEST_STATS, *TEST_RECIPE_CLASSICAL);
}

struct NoUseRng;

impl RngCore for NoUseRng {
    fn next_u32(&mut self) -> u32 {
        panic!("RNG was used when it should not have been")
    }

    fn next_u64(&mut self) -> u64 {
        panic!("RNG was used when it should not have been")
    }

    fn fill_bytes(&mut self, _dest: &mut [u8]) {
        panic!("RNG was used when it should not have been")
    }

    fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
        panic!("RNG was used when it should not have been")
    }
}

#[test]
fn verify_recipe_modifiers_classical() {
    assert_eq!(
        TEST_RECIPE_CLASSICAL.max_durability, 70,
        "Classical gear should have 70 durability, got {}",
        TEST_RECIPE_CLASSICAL.max_durability
    );
    assert_eq!(
        TEST_RECIPE_CLASSICAL.max_progress, 3900,
        "Classical gear should have 70 durability, got {}",
        TEST_RECIPE_CLASSICAL.max_progress
    );
    assert_eq!(
        TEST_RECIPE_CLASSICAL.max_quality, 10920,
        "Classical gear should have 70 durability, got {}",
        TEST_RECIPE_CLASSICAL.max_quality
    );
}
