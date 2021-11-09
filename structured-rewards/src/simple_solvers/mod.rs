//! This contains a very basic implementation of Structural Value Iteration as [`simple_solver`],
//! as well as a few traits necessary to get a basic environment together to run it.
//!
//! It is not recommended to use the implementation in this crate for serious work, only as
//! a quick and dirty testing ground for ideas.

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    mem,
};

use crate::{prelude::*, Sem};

#[cfg(feature = "environments")]
pub mod environments;

/// Defines the current place the agent/solver is evaluating. The general loop is that
/// [`actions`](State::actions) will provide an iterator over anything that will yield
/// a non-zero number of successor states (essentially "allowed" actions), and then
/// [`successors`](State::successors) will yield those states, along with the
/// [`TransitionReward`] for this domain (in Value Iteration this includes the transition
/// probability, in Q-learning this iterator would always have a single element and no
/// probability).
///
/// This is a very broad definition of a state space, and when writing your own solver
/// you may want to use something less abstract to better exploit your problem.
pub trait State<Q: QVal, R: TransitionReward<Q, P>, P: PartialQ<Q>>: Sized {
    /// An iterator over (s',R(s,a,s')), as returned by `successors`, if this is
    /// meant for an approximate learning solver like Q-learning, this should always
    /// have at most one element (unless the state is terminal), and `R(s,a)` should
    /// not encode a probability.
    type SuccRewardIter: Iterator<Item = (Self, R)>;
    /// The set of all valid actions for this domain
    type Action;
    /// An iterator over [`Self::Action`](State::Action), listing all
    /// the ones valid in the current state.
    type ActionIter: Iterator<Item = Self::Action>;

    /// Yields all (s',R(s,a,s')) from this state, or a single-element iterator
    /// with an observation, depending on the intended use for this [`State`].
    fn successors(&self, action: Self::Action) -> Self::SuccRewardIter;

    /// Yields an iterator over all valid actions for the current state.
    fn actions(&self) -> Self::ActionIter;
}

/// Determines if the Q-value estimate has converged in Value Iteration, generally
/// by checking how different the old and new estimates are in terms that make sense
/// with your [`QVal`] structure (for traditional floating point rewards this may be checking that no
/// values have increased beyond some small delta).
///
/// However, if desired you could also ignore it entirely and
/// simply keep track of the number of calls and cut off after a given number, for
/// instance.
pub trait ConvergenceMeasure<S: Hash, A: Hash, Q: QVal> {
    fn converges(
        &mut self,
        old: &HashMap<S, HashMap<A, Q>>,
        curr: &HashMap<S, HashMap<A, Q>>,
    ) -> bool;
}

/// An extremely simple implementation of Structural Value Iteration, which abstracts over traditional numeric
/// reward types into arbitrary structural ones defined by [`QVal`]. The general algorithm performs the update
///
/// ```notrust
/// Q(s,a) = tree_s'[R(s,a,s') ∘ max_a' Q(s',a')]
/// ```
///
/// Where `tree` is defined as a repeated call to [`partial_update`](Bellman::partial_update),
/// starting with the [`Default::default`] of [`PartialQ`] on the result of
/// [`TransitionReward`] being [`.compose`](Compose::compose)d with the [`QVal`] of the next state
/// estimate. It is then stored as an update to the current estimate by calling [`reweight`](Bellman::reweight).
///
/// This is done for all `Q(s,a)` reachable from `start`, as a single iteration, and then repeated
/// until the provided convergence measure `measure` returns `true`.
///
/// The initial estimate for each `Q(s,a)` will be the [`Default::default`] of [`QVal`].
///
/// Since this is an exact solver, the [`TransitionReward`] returned via [`successors`](State::successors)
/// should include the transition probability `T(s,a,s')` and the [`compose`](Compose::compose) implementation
/// should take this into account.
///
/// It is **not** recommended to use this as a general solver. It is highly unoptimized and cannot leverage
/// anything about your domain. For instance, even the testing environment for this crate, the GridWorld,
/// is not well represented in this, because the states can be represented as, well, a grid-shaped array or [`BTreeMap`]
/// much more efficiently for storing and retrieving the Q-values, but we're forced to use [`HashMap`] to
/// be as general as possible.
///
/// [`BTreeMap`]: std::collections::BTreeMap
pub fn simple_solver<S, R, Q, P, C>(start: S, mut measure: C) -> HashMap<S, HashMap<S::Action, Q>>
where
    S: State<Q, R, P> + Eq + Hash + Sized + Clone + std::fmt::Debug,
    R: TransitionReward<Q, P> + std::fmt::Debug,
    P: PartialQ<Q> + Default + std::fmt::Debug,
    S::Action: Eq + Hash + Sized + Clone + Copy + std::fmt::Debug,
    Q: QVal + Sized + std::fmt::Debug,
    C: ConvergenceMeasure<S, S::Action, Q>,
{
    let mut seen = HashSet::new();
    seen.insert(start.clone());
    let mut next_qs = HashMap::new();
    let mut stationary_qs = HashMap::new();

    loop {
        let mut stack = vec![start.clone()];
        while let Some(state) = stack.pop() {
            for action in state.actions() {
                let mut tree = P::default();
                for (succ, reward) in state.successors(action) {
                    let future_estimate = reward.compose(
                        get_best_action(&succ, succ.actions(), &stationary_qs)
                            .unwrap_or(&Q::default()),
                    );

                    tree = tree.partial_update(&future_estimate);
                    if !seen.contains(&succ) {
                        seen.insert(succ.clone());
                        stack.push(succ);
                    }
                }

                *get_state_action_mut(&state, action, &mut next_qs) = tree.reweight();
            }
        }

        if measure.converges(&stationary_qs, &next_qs) {
            break next_qs;
        } else {
            mem::swap(&mut next_qs, &mut stationary_qs);
            seen.clear();
        }
    }
}

fn get_state_action_mut<'a, K1: Eq + Hash + Clone, K2: Eq + Hash + Clone + Copy, V: Default>(
    state: &K1,
    action: K2,
    map: &'a mut HashMap<K1, HashMap<K2, V>>,
) -> &'a mut V {
    // Can't do `if let` cuz the borrow extends into the else :(
    if map.contains_key(state) {
        map.get_mut(state).unwrap().entry(action)
    } else {
        map.entry(state.clone())
            .or_insert_with(HashMap::new)
            .entry(action)
    }
    .or_default()
}

fn get_state_action<'a, K1: Eq + Hash + Clone, K2: Eq + Hash + Clone + Copy, V: Default>(
    state: &K1,
    action: K2,
    map: &'a HashMap<K1, HashMap<K2, V>>,
) -> Option<&'a V> {
    // Can't do `if let` cuz the borrow extends into the else :(
    if let Some(inner) = map.get(state) {
        inner.get(&action)
    } else {
        None
    }
}

fn get_best_action<
    'a,
    K1: Eq + Hash + Clone,
    K2: Eq + Hash + Clone + Copy,
    I: Iterator<Item = K2>,
    V: Default + SemanticOrd + SemanticEq,
>(
    state: &K1,
    actions: I,
    map: &'a HashMap<K1, HashMap<K2, V>>,
) -> Option<&'a V> {
    actions
        .map(|a| get_state_action(state, a, map).map(|v| v.sem()))
        .max()
        .flatten()
        .map(|Sem(v)| v)
}
