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
            assert_eq!(Manipulation(0).repair(), 0);
        }

        #[test]
        fn repair_active() {
            assert_eq!(Manipulation(1).repair(), Manipulation::REPAIR_VALUE);
        }

        #[test]
        fn activate() {
            test_activate(Manipulation(0));
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(Manipulation(0));
        }

        #[test]
        fn decay() {
            test_decay(Manipulation(0), true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(Manipulation(0), true)
        }

        #[test]
        fn reactivate() {
            test_reactivate(Manipulation(1), true);
        }
    }

    mod waste_not {
        use std::num::NonZeroU8;

        use super::*;
        use crate::buffs::durability::WasteNot;

        #[test]
        fn cost_mod_inactive() {
            assert_eq!(WasteNot::Inactive.durability_cost_mod(), 100);
        }

        #[test]
        fn cost_mod_active() {
            assert_eq!(
                WasteNot::WasteNot(NonZeroU8::new(2).unwrap()).durability_cost_mod(),
                WasteNot::DISCOUNT
            );
        }

        #[test]
        fn cost_mod_active_2() {
            assert_eq!(
                WasteNot::WasteNot2(NonZeroU8::new(2).unwrap()).durability_cost_mod(),
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
            test_reactivate(WasteNot::WasteNot(NonZeroU8::new(1).unwrap()), true);
        }

        #[test]
        fn waste_not_2_logic() {
            assert_eq!(
                WasteNot::Inactive.activate(0),
                WasteNot::WasteNot(NonZeroU8::new(WasteNot::BASE_DURATION).unwrap())
            );
            assert_eq!(
                WasteNot::Inactive.activate(2),
                WasteNot::WasteNot(NonZeroU8::new(WasteNot::BASE_DURATION + 2).unwrap())
            );
            assert_eq!(
                WasteNot::Inactive.activate(4),
                WasteNot::WasteNot2(NonZeroU8::new(WasteNot::BASE_DURATION + 4).unwrap())
            );
            assert_eq!(
                WasteNot::Inactive.activate(6),
                WasteNot::WasteNot2(NonZeroU8::new(WasteNot::BASE_DURATION + 6).unwrap())
            );
        }
    }
}

mod misc {
    use super::*;

    mod specialist {
        use crate::buffs::misc::SpecialistActions;

        #[cfg(debug_assertions)]
        #[test]
        #[should_panic(expected = "Too many delineations")]
        fn too_many() {
            SpecialistActions::Availalble(5).actions_available();
        }

        #[cfg(debug_assertions)]
        #[test]
        #[should_panic(expected = "Shouldn't be marked as available with 0 delineations")]
        fn bad_zero() {
            SpecialistActions::Availalble(0).actions_available();
        }

        #[test]
        fn should_be_available() {
            assert!(SpecialistActions::Availalble(3).actions_available());
        }

        #[test]
        fn shouldnt_be_available_out() {
            assert!(!SpecialistActions::Unavailable.actions_available());
        }

        #[test]
        fn shouldnt_be_available_no_specialist() {
            assert!(!SpecialistActions::NotSpecialist.actions_available());
        }

        #[test]
        fn shouldnt_be_unavailable() {
            assert!(!SpecialistActions::Availalble(3).actions_unavailable());
        }

        #[test]
        fn should_be_unavailable_out() {
            assert!(SpecialistActions::Unavailable.actions_unavailable());
        }

        #[test]
        fn should_be_unavailable_no_specialist() {
            assert!(SpecialistActions::NotSpecialist.actions_unavailable());
        }

        #[test]
        fn run_out() {
            assert_eq!(
                SpecialistActions::Availalble(1) - 1,
                SpecialistActions::Unavailable
            );
        }

        #[test]
        fn still_available() {
            assert_eq!(
                SpecialistActions::Availalble(2) - 1,
                SpecialistActions::Availalble(1)
            );
        }

        #[cfg(debug_assertions)]
        #[test]
        #[should_panic(expected = "Action should only use one delineation at a time")]
        fn bad_subtract() {
            let _ = SpecialistActions::Availalble(2) - 2;
        }
    }

    mod heart_and_soul {
        use super::*;
        use crate::buffs::misc::HeartAndSoul;

        // Since Heart and Soul has no duration (it lasts until used up), it's not a durational buff
        // so we have to reimplement some of the tests

        #[test]
        fn activate() {
            assert_eq!(HeartAndSoul::Inactive.activate(), HeartAndSoul::Active);
        }

        #[test]
        fn activate_in_place() {
            let mut has = HeartAndSoul::Inactive;
            has.activate_in_place();

            assert!(has.is_active());
        }

        #[test]
        fn is_active() {
            assert!(HeartAndSoul::Active.is_active());
        }

        #[test]
        fn is_not_active() {
            assert!(!HeartAndSoul::Inactive.is_active());
        }

        #[test]
        fn is_inactive() {
            assert!(HeartAndSoul::Inactive.is_inactive());
        }

        #[test]
        fn is_not_inactive() {
            assert!(!HeartAndSoul::Active.is_inactive());
        }

        #[test]
        fn deactivate() {
            test_deactivate(HeartAndSoul::Active, u8::MAX);
        }

        #[test]
        fn deactivate_in_place() {
            test_deactivate_in_place(HeartAndSoul::Active, u8::MAX);
        }

        #[cfg(debug_assertions)]
        #[test]
        #[should_panic(expected = "Attempted to deactivate deactivated HeartAndSoul")]
        fn deactivate_panic() {
            test_deactivate_panic(HeartAndSoul::Inactive);
        }

        #[cfg(debug_assertions)]
        #[test]
        #[should_panic(expected = "Attempted to deactivate deactivated HeartAndSoul")]
        fn deactivate_in_place_panic() {
            test_deactivate_in_place_panic(HeartAndSoul::Inactive);
        }
    }
}

mod progress {
    use super::*;

    mod veneration {
        use super::*;
        use crate::buffs::progress::Veneration;

        #[test]
        fn activate() {
            test_activate(Veneration(0));
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(Veneration(0));
        }

        #[test]
        fn decay() {
            test_decay(Veneration(0), true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(Veneration(0), true)
        }

        #[test]
        fn reactivate() {
            test_reactivate(Veneration(1), true);
        }
    }

    mod muscle_memory {
        use super::*;
        use crate::buffs::progress::MuscleMemory;

        #[test]
        fn activate() {
            test_activate(MuscleMemory(0));
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(MuscleMemory(0));
        }

        #[test]
        fn decay() {
            test_decay(MuscleMemory(0), true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(MuscleMemory(0), true)
        }

        #[test]
        fn deactivate() {
            test_deactivate_helper(MuscleMemory(0));
        }

        #[test]
        fn deactivate_in_place() {
            test_deactivate_in_place_helper(MuscleMemory(0));
        }

        #[test]
        #[should_panic(expected = "Attempt to deactivate inactive MuscleMemory")]
        fn deactivate_panic() {
            test_deactivate_panic(MuscleMemory(0));
        }

        #[test]
        #[should_panic(expected = "Attempt to deactivate inactive MuscleMemory")]
        fn deactivate_in_place_panic() {
            test_deactivate_in_place_panic(MuscleMemory(0));
        }
    }

    mod final_appraisal {
        use super::*;
        use crate::buffs::progress::FinalAppraisal;

        #[test]
        fn activate() {
            test_activate(FinalAppraisal(0));
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(FinalAppraisal(0));
        }

        #[test]
        fn decay() {
            test_decay(FinalAppraisal(0), true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(FinalAppraisal(0), true)
        }

        #[test]
        fn deactivate() {
            test_deactivate_helper(FinalAppraisal(0));
        }

        #[test]
        fn deactivate_in_place() {
            test_deactivate_in_place_helper(FinalAppraisal(0));
        }

        #[test]
        #[should_panic(expected = "Attempt to deactivate inactive FinalAppraisal")]
        fn deactivate_panic() {
            test_deactivate_panic(FinalAppraisal(0));
        }

        #[test]
        #[should_panic(expected = "Attempt to deactivate inactive FinalAppraisal")]
        fn deactivate_in_place_panic() {
            test_deactivate_in_place_panic(FinalAppraisal(0));
        }
    }
}

mod quality {
    use super::*;

    mod great_strides {
        use super::*;
        use crate::buffs::quality::GreatStrides;

        #[test]
        fn activate() {
            test_activate(GreatStrides(0));
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(GreatStrides(0));
        }

        #[test]
        fn decay() {
            test_decay(GreatStrides(0), true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(GreatStrides(0), true)
        }

        #[test]
        fn deactivate() {
            test_deactivate_helper(GreatStrides(0));
        }

        #[test]
        fn deactivate_in_place() {
            test_deactivate_in_place_helper(GreatStrides(0));
        }

        #[test]
        #[should_panic(expected = "Attempt to deactivate inactive GreatStrides")]
        fn deactivate_panic() {
            test_deactivate_panic(GreatStrides(0));
        }

        #[test]
        #[should_panic(expected = "Attempt to deactivate inactive GreatStrides")]
        fn deactivate_in_place_panic() {
            test_deactivate_in_place_panic(GreatStrides(0));
        }
    }

    mod innovation {
        use super::*;
        use crate::buffs::quality::Innovation;

        #[test]
        fn activate() {
            test_activate(Innovation(0));
        }

        #[test]
        fn activate_in_place() {
            test_activate_in_place(Innovation(0));
        }

        #[test]
        fn decay() {
            test_decay(Innovation(0), true)
        }

        #[test]
        fn decay_in_place() {
            test_decay_in_place(Innovation(0), true)
        }
    }

    mod inner_quiet {
        /* Left blank, it makes more sense for now to test this under actions since the observable effect is in context */
    }
}
