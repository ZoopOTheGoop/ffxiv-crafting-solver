use crate::{
    actions::progress::*,
    buffs::{self},
};

use super::ActionTester;

#[test]
fn basic_synthesis() {
    ActionTester::make(BasicSynthesis, "Basic Synthesis", None)
        .had_effect()
        .modified_cp(0)
        .passed_time(true)
        // Just for one synthesis action, make sure IQ isn't triggered
        .triggered_buff(buffs::quality::InnerQuiet::default(), |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(-10)
        .added_progress(273)
        // Again, just for one synthesis action, make sure quality isn't affected
        .added_quality(0);
}
