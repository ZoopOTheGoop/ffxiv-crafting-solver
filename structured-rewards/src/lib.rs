//! This crate contains definitions for working with Structured (or Structural) Rewards, which is an
//! extension of the notion of "reward" in Reinforcement Learning to arbitrary types, with implementor-defined
//! orderings and combination rules.
//!
#![cfg_attr(
    not(feature = "solvers"),
    doc = r#"By activating the 'solvers' feature, you can also gain access to a very simple,
    general solver for testing purposes.

"#
)]
#![cfg_attr(
    feature = "solvers",
    doc = r#"This crate also implements a very simple [solver](solvers::simple_solver) for testing purposes.
    However, the environment is only really useful for playing with the concept at a broad level,
    and the solver is a very generic implementation of Value Iteration, albeit extended to Structured Rewards,
    and likely unsuitable for complex domains, especially since structural rewards
    can help you exploit your domain better while optimizing.

"#
)]
#![cfg_attr(
    not(feature = "environments"),
    doc = "You can also gain access to some environments with the `environments` feature flag."
)]
#![cfg_attr(
    feature = "environments",
    doc = r#"There are also simple test environments in the [environments](solvers::environments) module,
    primarily useful for use with the [solver](solvers::simple_solver), and can be played with to get
    a very basic feel for how the API and rewards work.  
"#
)]
//!
//! In the future this should link to a more thorough overview of the concept.

#![warn(missing_docs)]

pub mod sem;

pub mod rewards;
#[cfg(feature = "solvers")]
pub mod simple_solvers;

pub use sem::{Sem, SemanticEq, SemanticOrd};

/// Contains the basic traits required to implement a generalized algorithm on structured rewards.
/// This merely provides a structured framework for defining your types. While it is recommended
/// you implement your own domain-specific agent training loops that can exploit your reward
/// and state structure, which would obviate the need for them, these should always be sufficient
/// for the overriding type framework and may provide guidance in design.
pub mod prelude {
    pub use crate::{
        Bellman, Compose, PartialQ, QVal, SemanticEq, SemanticOrd, TotalQ, TransitionReward,
    };
}

/// A `QVal` is the eventual estimate per state and action a Value-based Reinforcement
/// Learning agent will use to estimate the proficiency of taking an action. The only required
/// traits for `Qval` are [`Default`] and [`SemanticOrd`]. `Default` should be interpretive as the
/// "initial" value a Q-value should be initialized to in a solver (e.g. when creating the value table).
///
/// Unlike many other marker traits in this crate, `QVal` is not auto-implemented for types that implement
/// its requirements, and is opt-in, because to be a `QVal`, in addition to its trait requirements, it must
/// also be the type parameter of some [`PartialQ`]. If a `QVal` represents its own [`PartialQ`] it is automatically
/// [`TotalQ`].
///
/// Note that `QVal` is used a little loosely because type-wise it represents the same value as the value function,
/// `V`, and in many cases (such as simple floats), may be the same type as `R`, represented in this crate by the
/// [`TransitionReward`] type. This decision is due to the fact that abstracting over the reinforcement learning
/// concept of rewards and values requires concessions to the notion of how Q-values are actually processed in
/// various algorithms. More of this nuance is discussed in the documentation for [`TransitionReward`] and [`PartialQ`]
/// in particular.
pub trait QVal: Default + SemanticOrd {}

/// A TransitionReward represents the precise structural value that will be emitted by `R(s,a)` or `R(s,a,s')`
/// in a typical RL optimizer/agent training loop. In a situation such as value iteration, this is generally
/// assumed to be the union of `T(s,a,s')` and `R(s,a,s')`, whereas in Q-learning or SARSA, this is more likely
/// to simply be an observation of a concrete reward and in those case would not be uncommon for it to simply
/// be the same type as the associated [`QVal`].
///
/// This takes two generic types, `Q`, representing the [`QVal`] structure of the domain, and `P` representing
/// an intermediate [`PartialQ`]. This type encodes a transformation cycle, such that the element of this type
/// is [`Compose`]d with the best [`QVal`] estimate of the next state, and then these values are updated (via
/// the [`Bellman`] trait), into the [`QVal`] estimate of the current state, action pair.
///
/// This type is more likely to be implemented on a different struct than [`QVal`] in a situation where you're
/// doing an exact solver, such as value iteration, as since the notational updates to Structured Rewards encode
/// the transition function into the observed reward in these cases, since the purpose of [`QVal`] is to manage
/// *expectation* over reward trajectories. In cases such as Q-learning, it's very likely that this can simply
/// be the same type as its associated [`QVal`], and simply be reduced to an observed Q with certainty
/// 100%.
///
/// This trait is included in the [`prelude`] and need not be manually implemented, as long as the proper
/// implementations exist for [`Compose`], [`Bellman`], [`SemanticOrd`], and [`QVal`] exist it will be
/// done via blanket impl.
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

/// A `PartialQ` represents an under-construction Q-value. This is primarily useful
/// in Value Iteration, where all transitions for a state, action pair are observed
/// at once, and a [Q-value](QVal) may need to be constructed "from parts" when using non-numeric
/// rewards. For instance, if your [`QVal`] represents an expectation tree over unit-labels,
/// a [`PartialQ`] is able to take all the direct transition probabilities from a [`TransitionReward`]
/// that is [`Compose`]d with the next state's best [`QVal`] and assembles them into
/// a single structure represented by a struct implementing this trait, verifying the probabilities
/// are roughly correct (e.g. sum to `100%`) and simplifying before becoming an actual [`QVal`]
/// as used by the current (state, action) estimate.
///
/// It is very likely that these operations can be done within the [`QVal`] itself, and it is not
/// a problem for them to be the same type; this is called a [`TotalQ`]. However, the option exists
/// to split them up for convenience or a case where it's more efficient computationally to do so.
///
/// Note that in generalized algorithms that may require a [`PartialQ`] to function on an arbitrary
/// domain, they will often require a [`Default`] implementation which represents a zero-type which
/// will be built-up as the "seed" or "starting" value of [`self`] in
/// the first call to [`partial_update`](Bellman::partial_update).
///
/// This trait is included in the [`prelude`] and need not be manually implemented, as long as the proper
/// implementations exist for [`Compose`], [`Bellman`], [`SemanticOrd`], and [`QVal`] exist it will be
/// done via blanket impl.
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

/// A `TotalQ` is simply a [`QVal`] that is its own [`PartialQ`]. This represents a structure
/// that is either able to manage transition probability+reward pairings emitted by a [`TransitionReward`]
/// in exact solvers such as Value Iteration, does not need to be able to do that (i.e. is only going to be used
/// in a context such as Q-learning where this isn't an issue), or represents some type such as a strict numeric
/// reward like traditional MDP rewards whose partial update is more or less equivalent to its update.
///
/// This trait is included in the [`prelude`] and need not be manually implemented, as long as the proper
/// implementations exist for [`Compose`], [`Bellman`], [`SemanticOrd`], and [`QVal`] exist it will be
/// done via blanket impl.
pub trait TotalQ: QVal + PartialQ<Self> {}

impl<T> TotalQ for T where T: QVal + PartialQ<Self> {}

/// `Bellman` is a trait which represents the ability of a type to manage the expectation
/// over reward when receiving value updates in a value-based Reinforcement Learning algorithm.
///
/// This trait takes three generic types, which are assumed to be, but may not be, `Self`. This
/// trait is meant to form a transformation cycle with [`Compose`], where [`Compose::compose`]'s output
/// is meant to become the `Rhs` for this type, aka the `other` argument to [`update`](`Bellman::update`) or
/// [`partial_update`](`Bellman::partial_update`) depending on the algorithm.
/// Then, after calling [`reweight`](`Bellman::reweight`) (if [`partial_update`](`Bellman::partial_update`)
/// was used), or simply passing out of [`update`](`Bellman::update`), it will become the same type as the
/// `Rhs` (`other`) of [`Compose::compose`] for the previous state.
///
/// This cycle is represented by the [`TransitionReward`], [`PartialQ`], [`QVal`] triad. Generally speaking,
/// a [`TransitionReward`] is [`Compose`]d with a [`QVal`] to create a [`PartialQ`], which, after using
/// [`update`](`Bellman::update`) or [`partial_update`](`Bellman::partial_update`) and
/// [`reweight`](`Bellman::reweight`), will come to represent a [`QVal`] again.
///
/// This trait is included in the [`prelude`], but needs to be manually implemented. If this and
/// [`Compose`] are implemented on [`QVal`] correctly according to the transformation cycle
/// outlined in the paragraph above, [`PartialQ`] and [`TransitionReward`] will be automatically
/// implemented.
pub trait Bellman<Rhs = Self, Output = Self, Final = Self> {
    /// Performs a point observation on `other`, updating its expectation
    /// accordingly. This will immediately output the associated [`QVal`].
    fn update(&self, other: &Rhs) -> Final;

    /// Performs a partial update of the expectation, building up a sequence
    /// as in the `sum` term of Value Iteration (denoted `tree` in Structured
    /// Rewards).
    fn partial_update(&self, other: &Rhs) -> Output;

    /// Used to finalize to a [`QVal`] after repeated calls to
    /// [`partial_update`](Bellman::partial_update), making sure the expectation
    /// properly adds to 100% in cases where that's tracks. This may also perform
    /// simplification if desired.
    ///
    /// For some types (such as [`SimpleQ`](rewards::SimpleQ)) this is just a no-op.
    fn reweight(&self) -> Final;
}

/// A `Compose` represents a type which is combined with the next state's [`QVal`] estimate
/// to produce what is, in essence, a union of the reward for moving to the next state, combined
/// with the estimated reward for following the trajectory according to the current policy estimate
/// in that state.
///
/// This trait's output is such that when [`compose`](Compose::compose) is called with its `Rhs` being
/// a [`QVal`], it will emit a [`PartialQ`] which, after being processed via [`Bellman`] with further
/// [`PartialQ`] values (with this output being on the `Rhs` of those calls), finalizes back into the [`QVal`]
/// for that state, usable by other states updating their estimates.
pub trait Compose<Rhs = Self, Output = Self> {
    /// Composes this with `other`, yielding a [`PartialQ`].
    ///
    /// See the documentation for [`Compose`] and [`TransitionReward`]
    /// for more detailed information.
    fn compose(&self, other: &Rhs) -> Output;
}
