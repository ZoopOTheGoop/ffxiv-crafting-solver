mod legacy;
mod solver;

mod prelude {
    pub use crate::{Bellman, Composable, QVal, Reward};
}

pub trait Reward<Q: QVal<Self>>: Composable<Q> + Sized {}

pub trait QVal<R: Reward<Self>>: Ord + Bellman<R> + Mix + Difference + Sized {}

impl<Q, R: Reward<Q>> QVal<R> for Q where Q: Ord + Bellman<R> + Mix + Difference {}
impl<R, Q: QVal<R>> Reward<Q> for R where R: Composable<Q> {}

pub trait Mix {
    fn mix(&self, other: &Self) -> Self;
}

pub trait Difference {
    fn diff(&self, other: &Self) -> Self;
}

pub trait Bellman<R> {
    fn update(&self, other: &R) -> Self;
}

pub trait Composable<T> {
    fn compose(&self, other: &T) -> T;
}
