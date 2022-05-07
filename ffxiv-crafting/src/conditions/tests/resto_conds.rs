use super::*;
const CONDITIONS: [RestoExpertConditions; 6] = [
    RestoExpertConditions::Normal,
    RestoExpertConditions::Good,
    RestoExpertConditions::Pliant,
    RestoExpertConditions::Sturdy,
    RestoExpertConditions::Malleable,
    RestoExpertConditions::Primed,
];

mod codegen {
    use std::iter;

    use crate::conditions::tables::EXPERT_CRAFT_2;

    use super::*;

    #[test]
    fn condition_bits() {
        assert_eq!(<RestoExpertConditions as Condition>::BITS.0, EXPERT_CRAFT_2);
    }

    #[test]
    fn expert() {
        assert!(<RestoExpertConditions as Condition>::EXPERT);
    }

    #[test]
    fn is_excellent() {
        let truth = CONDITIONS
            .into_iter()
            .zip(iter::repeat(false))
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.is_excellent()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }

    /* There's probably some way to clean this up and reuse this code but it's more annoying than atrocious */

    #[test]
    fn is_good() {
        let truth = CONDITIONS
            .into_iter()
            .map(|cond| (cond, matches!(cond, RestoExpertConditions::Good)))
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.is_good()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }

    #[test]
    fn to_quality_modifier() {
        let truth = CONDITIONS
            .into_iter()
            .map(|cond| {
                (
                    cond,
                    if matches!(cond, RestoExpertConditions::Good) {
                        QualityModifier::Good
                    } else {
                        QualityModifier::Normal
                    },
                )
            })
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.to_quality_modifier()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }

    #[test]
    fn to_progress_modifier() {
        let truth = CONDITIONS
            .into_iter()
            .map(|cond| {
                (
                    cond,
                    if matches!(cond, RestoExpertConditions::Malleable) {
                        ProgressModifier::Malleable
                    } else {
                        ProgressModifier::Normal
                    },
                )
            })
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.to_progress_modifier()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }

    #[test]
    fn to_success_rate_modifier() {
        let truth = CONDITIONS
            .into_iter()
            .zip(iter::repeat(SuccessRateModifier::Normal))
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.to_success_rate_modifier()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }

    #[test]
    fn to_durability_modifier() {
        let truth = CONDITIONS
            .into_iter()
            .map(|cond| {
                (
                    cond,
                    if matches!(cond, RestoExpertConditions::Sturdy) {
                        DurabilityModifier::Sturdy
                    } else {
                        DurabilityModifier::Normal
                    },
                )
            })
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.to_durability_modifier()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }

    #[test]
    fn to_status_duration_modifier() {
        let truth = CONDITIONS
            .into_iter()
            .map(|cond| {
                (
                    cond,
                    if matches!(cond, RestoExpertConditions::Primed) {
                        StatusDurationModifier::Primed
                    } else {
                        StatusDurationModifier::Normal
                    },
                )
            })
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.to_status_duration_modifier()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }

    #[test]
    fn to_cp_usage_modifier() {
        let truth = CONDITIONS
            .into_iter()
            .map(|cond| {
                (
                    cond,
                    if matches!(cond, RestoExpertConditions::Pliant) {
                        CpUsageModifier::Pliant
                    } else {
                        CpUsageModifier::Normal
                    },
                )
            })
            .collect::<Vec<_>>();

        let reality = CONDITIONS
            .into_iter()
            .map(|cond| (cond, cond.to_cp_usage_modifier()))
            .collect::<Vec<_>>();

        assert_eq!(truth, reality);
    }
}

// It is apparently non-trivial to mock the Rng trait for non-u64 gen ranges :(
// so we'll just do a statistical analysis instead
#[test]
fn distribution() {
    // Seed from random.org
    let rng = StdRng::seed_from_u64(65342);

    let mut counts = HashMap::with_capacity(RAND_ANALYSIS_STEPS);
    let dist = RestoExpertConditions::default();

    for cond in dist.sample_iter(rng).take(RAND_ANALYSIS_STEPS) {
        *counts.entry(cond).or_insert(0) += 1usize;
    }

    assert!(
        counts.len() == CONDITIONS.len() && counts.values().all(|&v| v > 1),
        "Failed to observe all conditions at least once: {:?}",
        counts
    );

    let num_data = counts.values().sum::<usize>() as f64;

    let expected = HashMap::from([
        (RestoExpertConditions::Normal, 0.37),
        (RestoExpertConditions::Good, 0.12),
        (RestoExpertConditions::Pliant, 0.12),
        (RestoExpertConditions::Sturdy, 0.15),
        (RestoExpertConditions::Malleable, 0.12),
        (RestoExpertConditions::Primed, 0.12),
    ]);

    for (cond, avg) in counts.into_iter().map(|(k, v)| (k, v as f64 / num_data)) {
        let expect_p = expected.get(&cond).unwrap();

        assert!(
            (avg - expect_p).abs() < DISTRIBUTION_ANALYSIS_ALLOWED_ERROR,
            "p({:?}) = {} (wanted {})",
            cond,
            avg,
            expect_p
        )
    }
}
