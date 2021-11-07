use std::cmp::Ordering;

pub mod solver;

mod prelude {
    pub use crate::{Bellman, Compose, QVal, SemanticEq, SemanticOrd};
}

#[derive(Clone, Copy, Debug)]
pub struct Sem<T>(pub T);

impl<T> PartialEq for Sem<T>
where
    T: SemanticEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.sem_eq(&other.0)
    }
}

impl<T> Eq for Sem<T> where T: SemanticEq {}

impl<T> PartialOrd for Sem<T>
where
    T: SemanticOrd + SemanticEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.sem_cmp(&other.0))
    }
}

impl<T> Ord for Sem<T>
where
    T: SemanticOrd + SemanticEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.sem_cmp(&other.0)
    }
}

pub trait SemanticEq {
    fn sem_eq(&self, other: &Self) -> bool;

    fn sem(self) -> Sem<Self>
    where
        Self: Sized,
    {
        Sem(self)
    }
}

impl<T> SemanticEq for &T
where
    T: SemanticEq,
{
    fn sem_eq(&self, other: &Self) -> bool {
        (**self).sem_eq(*other)
    }
}

impl<T> SemanticEq for &mut T
where
    T: SemanticEq,
{
    fn sem_eq(&self, other: &Self) -> bool {
        (**self).sem_eq(&**other)
    }
}

pub trait SemanticOrd: SemanticEq {
    fn sem_cmp(&self, other: &Self) -> Ordering;
}

impl<T> SemanticOrd for &T
where
    T: SemanticOrd,
{
    fn sem_cmp(&self, other: &Self) -> Ordering {
        (**self).sem_cmp(*other)
    }
}

impl<T> SemanticOrd for &mut T
where
    T: SemanticOrd,
{
    fn sem_cmp(&self, other: &Self) -> Ordering {
        (**self).sem_cmp(&**other)
    }
}

pub trait IntoInner {
    type Inner;
    fn into_inner(self) -> Self::Inner;
}

pub trait QVal: Default + SemanticOrd {}

pub trait TransitionReward<Q, P>: Compose<Q, P> + Sized
where
    Q: QVal,
    P: PartialQ<Q>,
{
}

impl<T, Q, P> TransitionReward<Q, P> for T
where
    T: Compose<Q, P> + Sized,
    P: PartialQ<Q> + Sized,
    Q: QVal,
{
}

pub trait PartialQ<Q>: Bellman<Self, Self, Q> + Sized
where
    Q: QVal,
{
}

impl<Q, T> PartialQ<Q> for T
where
    T: Bellman<Self, Self, Q>,
    Q: QVal,
{
}

pub trait Bellman<Rhs = Self, Output = Self, Final = Self> {
    fn update(&self, other: &Rhs) -> Output;

    fn partial_update(&self, other: &Rhs) -> Output;

    fn reweight(&self) -> Final;
}

pub trait Compose<Rhs = Self, Output = Self> {
    fn compose(&self, other: &Rhs) -> Output;
}
