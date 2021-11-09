use crate::prelude::*;

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

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct NoDiscountReward(pub f64, pub i64);

impl Compose<SimpleQ, SimpleQ> for NoDiscountReward {
    fn compose(&self, other: &SimpleQ) -> SimpleQ {
        SimpleQ(self.0 * (self.1 as f64 + other.0))
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DiscountedReward<const D: i64>(pub f64, pub i64);

impl<const D: i64> Compose<SimpleQ, SimpleQ> for DiscountedReward<D> {
    fn compose(&self, other: &SimpleQ) -> SimpleQ {
        SimpleQ(self.0 * (self.1 as f64 + (D as f64 / 100.) * other.0))
    }
}
