use crate::prelude::*;
use std::{collections::HashMap, hash::Hash};

#[cfg(test)]
mod test;

pub trait State<R: Reward<Q>, A: Action, Q: QVal<R>>: Sized {
    type Iter: Iterator<Item = (Self, R)>;
    fn successors(&self, action: A) -> Self::Iter;
}

pub trait Action: Eq {}

pub trait ActionSet<S: State<R, A, Q>, A: Action, R: Reward<Q>, Q: QVal<R>> {
    type Iter: Iterator<Item = A>;

    fn filter(&self, curr_state: &S) -> Self::Iter;
    fn iter(&self) -> Self::Iter;
    fn count(&self) -> usize;
}

pub trait ConvergenceMeasure<S: State<R, A, Q>, A: Action, R: Reward<Q>, Q: QVal<R>> {
    fn converges(
        &mut self,
        old: &HashMap<S, HashMap<A, R>>,
        curr: &HashMap<S, HashMap<A, R>>,
    ) -> bool;
}

pub fn simple_solver<S, A, As, R, Q, C>(
    start: S,
    actions: As,
    mut measure: C,
) -> HashMap<S, HashMap<A, Q>>
where
    S: State<R, A, Q> + Hash + Eq + Clone,
    A: Action + Hash + Clone + Copy,
    As: ActionSet<S, A, R, Q>,
    R: Reward<Q> + Default + Clone + 'static,
    Q: QVal<R> + Default + Clone + 'static,
    C: ConvergenceMeasure<S, A, R, Q>,
{
    let mut stack = vec![start];
    let mut curr_vals = HashMap::new();
    let mut stationary_vals: Option<HashMap<S, HashMap<A, Q>>> = None;

    loop {
        while let Some(state) = stack.pop() {
            for action in actions.filter(&state) {
                let mut succ_reward = R::default();

                for (succ, reward) in state.successors(action) {
                    stack.push(succ);

                    let succ_reward = succ_reward.mix(&reward);
                }
                let q = if let Some(m) = curr_vals.get_mut(&state) {
                    m
                } else {
                    curr_vals
                        .entry(state.clone())
                        .or_insert_with(|| HashMap::with_capacity(actions.count()))
                }
                .entry(action)
                .or_insert_with(R::default);
                *q = q.update(&succ_reward);
            }
        }

        if stationary_vals
            .as_ref()
            .map(|v| measure.converges(v, &curr_vals))
            .unwrap_or(false)
        {
            break curr_vals;
        } else {
            let state_size = curr_vals.len();
            curr_vals = stationary_vals
                .replace(curr_vals)
                .unwrap_or_else(|| HashMap::with_capacity(state_size));
        }
    }
}
