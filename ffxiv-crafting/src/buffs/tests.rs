use super::*;
use std::fmt::Debug;

fn test_activate<D: DurationalBuff + Debug>(buff: D) {
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_activate_in_place`, buff needs to start inactive got: {:?}",
        buff
    );
    let activated = buff.activate(0);
    assert!(activated.is_active());
    assert!(
        !activated.is_inactive(),
        "Buff inactivity check not working properly"
    );
}

fn test_activate_in_place<D: DurationalBuff + Debug>(mut buff: D) {
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_activate_in_place`, buff needs to start inactive got: {:?}",
        buff
    );
    buff.activate_in_place(0);
    assert!(buff.is_active());
}

fn test_decay<D: DurationalBuff + Debug>(buff: D, obeys_bonus: bool) {
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_decay`, buff needs to start inactive got: {:?}",
        buff
    );

    let buff = buff.activate(2);
    let mut next = if obeys_bonus {
        let mut next = buff.decay();
        // We can be sure next is active for two more decays because of the bonus
        assert!(next.is_active());
        next = next.decay();
        assert!(next.is_active());
        next
    } else {
        buff
    };

    for _ in 0..D::BASE_DURATION {
        assert!(next.is_active());
        next = next.decay();
    }

    assert!(
        next.is_inactive(),
        "Buff did not deactivate at proper time: {:?}",
        next
    );
}

fn test_decay_in_place<D: DurationalBuff + Debug>(mut buff: D, obeys_bonus: bool) {
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_decay_in_place`, buff needs to start inactive got: {:?}",
        buff
    );

    buff.activate_in_place(2);
    if obeys_bonus {
        buff.decay_in_place();
        // We can be sure next is active for two more decays because of the bonus
        assert!(buff.is_active());
        buff.decay_in_place();
        assert!(buff.is_active());
    }

    for _ in 0..D::BASE_DURATION {
        assert!(buff.is_active());
        buff.decay_in_place();
    }

    assert!(
        buff.is_inactive(),
        "Buff did not deactivate at proper time: {:?}",
        buff
    );
}

fn test_deactivate<C: ConsumableBuff + Debug>(buff: C, expected_duration: u8) {
    assert!(
        buff.is_active(),
        "Setup failure in `test_deactivate`, buff needs to start active got: {:?}",
        buff
    );

    let (deactivated, remaining) = buff.deactivate();

    assert!(
        deactivated.is_inactive(),
        "Buff is not deactivated as expected: {:?}",
        deactivated
    );
    assert_eq!(
        remaining, expected_duration,
        "Duration not as expected; passed in buff was: {:?}; expected: {}; got: {}",
        buff, expected_duration, remaining
    );
}

fn test_deactivate_in_place<C: ConsumableBuff + Debug>(mut buff: C, expected_duration: u8) {
    assert!(
        buff.is_active(),
        "Setup failure in `test_deactivate_in_place`, buff needs to start active got: {:?}",
        buff
    );

    let remaining = buff.deactivate_in_place();

    assert!(
        buff.is_inactive(),
        "Buff is not deactivated as expected: {:?}",
        buff
    );
    assert_eq!(
        remaining, expected_duration,
        "Duration not as expected; expected: {}; got: {}",
        expected_duration, remaining
    );
}

fn test_deactivate_helper<B>(mut buff: B)
where
    B: ConsumableBuff + DurationalBuff + Debug,
{
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_deactivate_helper`, buff needs to start inactive got: {:?}",
        buff
    );

    buff.activate_in_place(0);
    test_deactivate(buff, B::BASE_DURATION);
}

fn test_deactivate_in_place_helper<B>(mut buff: B)
where
    B: ConsumableBuff + DurationalBuff + Debug,
{
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_deactivate_in_place_helper`, buff needs to start inactive got: {:?}",
        buff
    );

    buff.activate_in_place(0);
    test_deactivate_in_place(buff, B::BASE_DURATION);
}

#[cfg(debug_assertions)]
fn test_deactivate_panic<B: ConsumableBuff + Debug>(buff: B) {
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_deactivate_panic`, buff needs to start inactive got: {:?}",
        buff
    );

    buff.deactivate();
}

#[cfg(debug_assertions)]
fn test_deactivate_in_place_panic<B: ConsumableBuff + Debug>(buff: B) {
    assert!(
        buff.is_inactive(),
        "Setup failure in `test_deactivate_panic`, buff needs to start inactive got: {:?}",
        buff
    );

    buff.deactivate();
}

fn test_reactivate<D: DurationalBuff + Debug + Clone>(mut buff: D, obeys_bonus: bool) {
    buff.activate_in_place(0);
    assert!(buff.is_active());
    buff.activate_in_place(2);
    assert!(buff.is_active());

    // Asserts the reactivation with the bonus actually took
    if obeys_bonus {
        let mut count = 0;
        while buff.is_active() {
            count += 1;
            buff.decay_in_place();
        }

        assert_eq!(count, D::BASE_DURATION + 2);
    }
}

mod combo {
    use super::*;

    mod basic_touch {
        use super::*;
        use crate::buffs::combo::BasicTouchCombo;

        #[test]
        fn activate() {
            test_activate(BasicTouchCombo::Inactive);
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(BasicTouchCombo::Inactive);
        }

        #[test]
        fn decay() {
            test_decay(BasicTouchCombo::Inactive, false)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(BasicTouchCombo::Inactive, false)
        }

        #[test]
        fn reactivate() {
            test_reactivate(BasicTouchCombo::BasicTouch, false);
        }

        #[test]
        fn standard_touch_combo_activate() {
            assert_eq!(
                BasicTouchCombo::StandardTouch.activate(0),
                BasicTouchCombo::BasicTouch
            );
        }

        #[test]
        fn standard_touch_combo_decay() {
            assert_eq!(
                BasicTouchCombo::StandardTouch.decay(),
                BasicTouchCombo::Inactive
            );
        }
    }

    mod observe {
        use super::*;
        use crate::buffs::combo::ObserveCombo;

        #[test]
        fn activate() {
            test_activate(ObserveCombo::Inactive);
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(ObserveCombo::Inactive);
        }

        #[test]
        fn decay() {
            test_decay(ObserveCombo::Inactive, false)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(ObserveCombo::Inactive, false)
        }

        #[test]
        fn reactivate() {
            test_reactivate(ObserveCombo::Active, false);
        }
    }
}

mod durability {
    use super::*;
    mod manipulation {
        use super::*;
        use crate::buffs::durability::Manipulation;

        #[test]
        fn repair_inactive() {
            assert_eq!(Manipulation::Inactive.repair(), 0);
        }

        #[test]
        fn repair_active() {
            assert_eq!(Manipulation::Active(0).repair(), Manipulation::REPAIR_VALUE);
        }

        #[test]
        fn activate() {
            test_activate(Manipulation::Inactive);
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(Manipulation::Inactive);
        }

        #[test]
        fn decay() {
            test_decay(Manipulation::Inactive, true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(Manipulation::Inactive, true)
        }

        #[test]
        fn reactivate() {
            test_reactivate(Manipulation::Active(0), true);
        }
    }

    mod waste_not {
        use super::*;
        use crate::buffs::durability::WasteNot;

        #[test]
        fn cost_mod_inactive() {
            assert_eq!(WasteNot::Inactive.durability_cost_mod(), 100);
        }

        #[test]
        fn cost_mod_active() {
            assert_eq!(
                WasteNot::WasteNot(2).durability_cost_mod(),
                WasteNot::DISCOUNT
            );
        }

        #[test]
        fn cost_mod_active_2() {
            assert_eq!(
                WasteNot::WasteNot2(2).durability_cost_mod(),
                WasteNot::DISCOUNT
            );
        }

        #[test]
        fn activate() {
            test_activate(WasteNot::Inactive);
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(WasteNot::Inactive);
        }

        #[test]
        fn decay() {
            test_decay(WasteNot::Inactive, true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(WasteNot::Inactive, true)
        }

        #[test]
        fn reactivate() {
            test_reactivate(WasteNot::WasteNot(0), true);
        }

        #[test]
        fn waste_not_2_logic() {
            assert_eq!(
                WasteNot::Inactive.activate(0),
                WasteNot::WasteNot(WasteNot::BASE_DURATION)
            );
            assert_eq!(
                WasteNot::Inactive.activate(2),
                WasteNot::WasteNot(WasteNot::BASE_DURATION + 2)
            );
            assert_eq!(
                WasteNot::Inactive.activate(4),
                WasteNot::WasteNot2(WasteNot::BASE_DURATION + 4)
            );
            assert_eq!(
                WasteNot::Inactive.activate(6),
                WasteNot::WasteNot2(WasteNot::BASE_DURATION + 6)
            );
        }
    }
}
