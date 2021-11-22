//! Contains very simple concrete implementations of rewards.
//!
//! At the moment these just emulate traditional RL as a proof of concept that
//! traditional RL rewards can be reduced to Structured Rewards.

use crate::prelude::*;

/// A very simple numeric [Q-value](QVal) backed by an `f64`. This is essentially a
/// "traditional" Q-value. This is a [`TotalQ`] in terms of this crate's nomenclature.
///
/// In exact domains, this needs to be paired with a [`NoDiscountReward`] or a [`DiscountedReward`] depending
/// on whether or not you want discounting. The reason these are implemented on the reward,
/// despite being applied solely to the Q-value, is because the [`TransitionReward`] holds the [`Compose`]
/// implementation, and the choice of discounting is up to that.
///
/// Currently, no matching [`TransitionReward`] implementation exists for non-exact domains.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct SimpleQ(pub f64);

impl QVal for SimpleQ {}

impl Bellman for SimpleQ {
    fn update(&self, other: &Self) -> Self {
        SimpleQ(self.0 + other.0)
    }

    fn partial_update(&self, other: &Self) -> Self {
        self.update(other)
    }

    fn reweight(&self) -> Self {
        *self
    }
}

impl SemanticOrd for SimpleQ {
    fn sem_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl SemanticEq for SimpleQ {
    fn sem_eq(&self, other: &Self) -> bool {
        debug_assert!(!self.0.is_nan());
        debug_assert!(!other.0.is_nan());
        self.0 == other.0
    }
}

/// A [`TransitionReward`] for exact domains that does not apply any discount factor
/// when its [`compose`](Compose::compose) method is called.
///
/// The first element is the exact transition probability `T(s,a,s')` of the
/// (state, action, next-state) triad that emitted this reward, in the range
/// [0.0, 1.0].
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct NoDiscountReward(pub f64, pub i64);

impl Compose<SimpleQ, SimpleQ> for NoDiscountReward {
    fn compose(&self, other: &SimpleQ) -> SimpleQ {
        SimpleQ(self.0 * (self.1 as f64 + other.0))
    }
}

/// A [`TransitionReward`] for exact domains that applies a discount factor
/// when its [`compose`](Compose::compose) method is called.
///
/// The first element is the exact transition probability `T(s,a,s')` of the
/// (state, action, next-state) triad that emitted this reward.
///
/// Its discount factor is associated with the generic constant `D`.
/// Currently generic constants cannot be floating point types,
/// only [`bool`] or integral types, so `D` is divided by
/// `100` to get the actual factor.
///
/// Due to this being a constant, you'll need to hand implement
/// something yourself if you want something like hyperbolic discounting.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DiscountedReward<const D: i64>(pub f64, pub i64);

impl<const D: i64> Compose<SimpleQ, SimpleQ> for DiscountedReward<D> {
    fn compose(&self, other: &SimpleQ) -> SimpleQ {
        SimpleQ(self.0 * (self.1 as f64 + (D as f64 / 100.) * other.0))
    }
}
