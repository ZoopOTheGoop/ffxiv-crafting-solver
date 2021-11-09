use crate::{prelude::*, rewards::*};

pub trait SimpleReward: Sized + Compose<SimpleQ, SimpleQ> + Default + std::fmt::Debug {
    fn from_prob_val(prob: f64, val: i64) -> Self;
}

impl SimpleReward for NoDiscountReward {
    fn from_prob_val(prob: f64, val: i64) -> Self {
        Self(prob, val)
    }
}

impl<const D: i64> SimpleReward for DiscountedReward<D> {
    fn from_prob_val(prob: f64, val: i64) -> Self {
        Self(prob, val)
    }
}
