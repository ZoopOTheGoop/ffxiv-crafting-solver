use crate::{
    actions::{quality::*, CpCost, DurabilityFactor},
    buffs::{self},
};

use super::ActionTester;

#[test]
fn basic_touch() {
    ActionTester::make(BasicTouch, "Basic Touch", None)
        .had_effect()
        .used_cp(BasicTouch::CP_COST)
        .passed_time(true)
        .triggered_buff(buffs::combo::BasicTouchCombo::BasicTouch, |buffs| {
            buffs.combo.basic_touch
        })
        .triggered_buff(buffs::quality::InnerQuiet::default() + 1, |buffs| {
            buffs.quality.inner_quiet
        })
        .changed_durability(BasicTouch::DURABILITY_USAGE)
        .added_quality(252);
}
