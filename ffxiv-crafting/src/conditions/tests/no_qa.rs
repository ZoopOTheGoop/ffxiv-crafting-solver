use super::*;
const CONDITIONS: [NoQARegularConditions; 4] = [
    NoQARegularConditions::Normal,
    NoQARegularConditions::Good,
    NoQARegularConditions::Excellent,
    NoQARegularConditions::Poor,
];

mod codegen {
    use std::iter;

    use super::*;

    #[test]
    fn condition_bits() {
        assert_eq!(
            <NoQARegularConditions as Condition>::BITS.0,
            NORMAL_CONDITIONS
        );
    }

    #[test]
    fn expert() {
        assert!(!<NoQARegularConditions as Condition>::EXPERT);
    }

    #[test]
    fn is_excellent() {
        let truth = CONDITIONS
            .into_iter()
            .map(|cond| (cond, matches!(cond, NoQARegularConditions::Excellent)))
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
            .map(|cond| (cond, matches!(cond, NoQARegularConditions::Good)))
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
            .zip(
                [
                    QualityModifier::Normal,
                    QualityModifier::Good,
                    QualityModifier::Excellent,
                    QualityModifier::Poor,
                ]
                .into_iter(),
            )
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
            .zip(iter::repeat(ProgressModifier::Normal))
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
            .zip(iter::repeat(DurabilityModifier::Normal))
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
            .zip(iter::repeat(StatusDurationModifier::Normal))
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
            .zip(iter::repeat(CpUsageModifier::Normal))
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
    let mut rng = StdRng::seed_from_u64(65342);

    let mut counts = HashMap::with_capacity(RAND_ANALYSIS_STEPS);
    let mut prev = NoQARegularConditions::Normal;

    for _ in 0..RAND_ANALYSIS_STEPS {
        let cond = prev.sample(&mut rng);
        match prev {
            NoQARegularConditions::Excellent => assert_eq!(cond, NoQARegularConditions::Poor),
            NoQARegularConditions::Poor | NoQARegularConditions::Good => {
                assert_eq!(cond, NoQARegularConditions::Normal)
            }
            NoQARegularConditions::Normal => {
                *counts.entry(cond).or_insert(0) += 1usize;
            }
        }

        prev = cond;
    }

    assert!(
        counts.len() == 3 && counts.values().all(|&v| v > 1),
        "Failed to observe all conditions at least once: {:?}",
        counts
    );

    let num_data = counts.values().sum::<usize>() as f64;

    for (cond, avg) in counts.into_iter().map(|(k, v)| (k, v as f64 / num_data)) {
        match cond {
            // Allow ~0.1% error
            NoQARegularConditions::Excellent => {
                assert!(
                    (avg - 0.04).abs() < DISTRIBUTION_ANALYSIS_ALLOWED_ERROR,
                    "p(Excellent) = {}",
                    avg
                )
            }
            NoQARegularConditions::Good => {
                assert!(
                    (avg - 0.20).abs() < DISTRIBUTION_ANALYSIS_ALLOWED_ERROR,
                    "p(Good) = {}",
                    avg
                )
            }
            NoQARegularConditions::Normal => {
                assert!(
                    (avg - 0.76).abs() < DISTRIBUTION_ANALYSIS_ALLOWED_ERROR,
                    "p(Normal) = {}",
                    avg
                )
            }
            NoQARegularConditions::Poor => unreachable!("Poors shouldn't be added"),
        }
    }
}
