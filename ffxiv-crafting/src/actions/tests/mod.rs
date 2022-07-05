#![allow(missing_docs)]

mod buffs;
mod quality;

use std::fmt::Debug;

use lazy_static::lazy_static;
use rand::RngCore;

use crate::{
    actions::Action,
    buffs::{Buff, BuffState},
    conditions::QARegularConditions,
    quality_map::HQMap,
    recipe::{RLvl, Recipe},
    CharacterStats, CraftingSimulator, CraftingState,
};

use super::{ActionOutcome, StateDelta};

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

struct ActionTester<'a, A: Action + Copy> {
    #[allow(dead_code)]
    action: A,
    name: &'a str,
    state: CraftingState<'static, QARegularConditions, HQMap>,
    delta: StateDelta,
}

impl<'a, A: Action + Copy> ActionTester<'a, A> {
    fn make(
        action: A,
        name: &'a str,
        state: Option<CraftingState<'static, QARegularConditions, HQMap>>,
    ) -> Self {
        let state = state.unwrap_or_else(|| CraftingState::new_simulation(&CLASSICAL_SIMULATOR));

        let outcome = action.prospective_act(&state);
        let delta = match outcome {
            Ok(outcome) => match outcome {
                ActionOutcome::InProgress(delta) => delta,
                other => panic!("{} made craft fail or succeed: {:?}", name, other),
            },
            Err(err) => panic!("Error trying to execute {} in a valid state: {}", name, err),
        };

        Self {
            action,
            name,
            state,
            delta,
        }
    }

    fn used_cp(self, amount: i16) -> Self {
        let result = self.state + self.delta;

        assert_eq!(
            result.curr_cp,
            self.state.curr_cp - amount,
            "Applying {} does not cost/give {} CP;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
            self.name,
            amount,
            self.state,
            result,
            self.delta
        );

        self
    }

    fn used_delineation(self) -> Self {
        use crate::buffs::misc::SpecialistActions;

        let result = self.state + self.delta;

        let expected = match self.state.buffs.specialist_actions {
            SpecialistActions::Availalble(n @ 2..=u8::MAX) => SpecialistActions::Availalble(n - 1),
            SpecialistActions::Availalble(1) => SpecialistActions::Unavailable,
            SpecialistActions::Availalble(0) => unreachable!(),
            SpecialistActions::NotSpecialist => panic!(
                "Action {} should not execute - character is not specialist",
                self.name
            ),
            SpecialistActions::Unavailable => panic!(
                "Action {} should not execute - out of specialist actions",
                self.name
            ),
        };

        assert_eq!(
            result.buffs.specialist_actions, expected,
            "Applying {} does not cost a specialist action;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
            self.name,
            self.state,
            result,
            self.delta
        );

        self
    }

    #[allow(dead_code)]
    fn expected_changes<F>(self, alteration: F) -> Self
    where
        F: Fn(
            CraftingState<'static, QARegularConditions, HQMap>,
        ) -> CraftingState<'static, QARegularConditions, HQMap>,
    {
        let expected = alteration(self.state);
        let result = self.state + self.delta;

        assert_eq!(
            result, expected,
            "Applying {} did not change the correct things\n\texpected: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
            self.name,
            expected,
            result,
            self.delta,
        );

        self
    }

    fn had_effect(self) -> Self {
        let result = self.state + self.delta;
        assert_ne!(
            self.state, result,
            "Applying {} has no effect;\n\tstate: {:?}\n\tresult: {:?}\n\tdelta: {:?}",
            self.name, self.state, result, self.delta
        );

        self
    }

    fn passed_time(self, should_pass_time: bool) -> Self {
        if self.delta.time_passed && !should_pass_time {
            panic!(
                "Applying {} causes time to pass when it shouldn't;\n\tstate:{:?}, delta: {:?}",
                self.name, self.state, self.delta
            );
        } else if !self.delta.time_passed && should_pass_time {
            panic!(
                "Applying {} doesn't cause time to pass when it should;\n\tstate:{:?}, delta: {:?}",
                self.name, self.state, self.delta
            );
        }

        self
    }

    fn triggered_buff<B, F1>(self, buff: B, get_buff: F1) -> Self
    where
        B: Buff + Eq + Debug,
        F1: Fn(BuffState) -> B,
    {
        let result = self.state + self.delta;
        assert_eq!(get_buff(result.buffs), buff,
            "Applying {} does not cause buff {:?} to be applied;\n\tstate:{:?}, result: {:?}, delta: {:?}",
            self.name, buff, self.state, result, self.delta
        );

        self
    }

    fn changed_durability(self, change: i8) -> Self {
        let result = self.state + self.delta;

        assert_eq!(self.state.curr_durability + change, result.curr_durability,
            "Applying {} does not cause durability {} of {} to be applied;\n\tstate:{:?}, result: {:?}, delta: {:?}",
        self.name, if change <= 0 { "loss" } else { "gain" }, change, self.state, result, self.delta);

        self
    }

    fn added_quality(self, quality: u32) -> Self {
        let result = self.state + self.delta;

        assert_eq!(
            self.state.curr_quality, result.curr_quality,
            "Applying {} does not cause quality \
            to be increased by {};\n\tstate:{:?}, result: {:?}, delta: {:?}",
            self.name, quality, self.state, result, self.delta
        );

        self
    }
}
