use crate::{prelude::*, PartialQ, Sem, TransitionReward};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    mem,
};

#[cfg(test)]
mod test;

pub trait State<Q: QVal, R: TransitionReward<Q, P>, P: PartialQ<Q>>: Sized {
    type SuccRewardIter: Iterator<Item = (Self, R)>;
    type Action;
    type ActionIter: Iterator<Item = Self::Action>;

    fn successors(&self, action: Self::Action) -> Self::SuccRewardIter;

    fn actions(&self) -> Self::ActionIter;
}

pub trait ConvergenceMeasure<S: Hash, A: Hash, Q: QVal> {
    fn converges(
        &mut self,
        old: &HashMap<S, HashMap<A, Q>>,
        curr: &HashMap<S, HashMap<A, Q>>,
    ) -> bool;
}

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
