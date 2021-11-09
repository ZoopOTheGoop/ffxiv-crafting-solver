use std::collections::HashMap;

use crate::{
    rewards::SimpleQ,
    simple_solvers::{
        environments::{traits::SimpleReward, GridAction, GridState},
        ConvergenceMeasure,
    },
};

pub struct StandardConvergenceMeasure;

impl<'a, R, const N: usize> ConvergenceMeasure<GridState<'a, R, N>, GridAction, SimpleQ>
    for StandardConvergenceMeasure
where
    R: SimpleReward,
{
    fn converges(
        &mut self,
        old: &HashMap<GridState<'a, R, N>, HashMap<GridAction, SimpleQ>>,
        curr: &HashMap<GridState<'a, R, N>, HashMap<GridAction, SimpleQ>>,
    ) -> bool {
        for (state, qs) in curr.iter() {
            for (action, new_q) in qs.iter() {
                if let Some(old_q) = old.get(state).and_then(|v| v.get(action)) {
                    if (old_q.0 - new_q.0).abs() > 1e-8 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }
}
