use std::{
    borrow::Borrow,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use derivative::Derivative;

use crate::{rewards::SimpleQ, simple_solvers::State};

pub mod actions;
pub mod convergence;
pub mod traits;

#[cfg(test)]
mod test;

pub use actions::*;
use traits::SimpleReward;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GridWorld<const N: usize> {
    grid: [[(i64, RandomTransition, bool); N]; N],
}

impl<const N: usize> Default for GridWorld<N> {
    fn default() -> Self {
        let mut me = GridWorld {
            grid: [[Default::default(); N]; N],
        };
        me[(0, 0)] = (15, RandomTransition::None, true);
        me[(0, 1)] = (
            0,
            RandomTransition::RealAction {
                action: GridAction::West,
                prob: 0.2,
            },
            false,
        );
        me
    }
}

impl<const N: usize> Index<(usize, usize)> for GridWorld<N> {
    type Output = (i64, RandomTransition, bool);

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.1][index.0]
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for GridWorld<N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.1][index.0]
    }
}

#[derive(Derivative, Copy, Clone)]
#[derivative(Hash, PartialEq, Eq, Debug)]
pub struct GridState<'a, R, const N: usize>
where
    R: SimpleReward,
{
    #[derivative(Hash = "ignore", PartialEq = "ignore", Debug = "ignore")]
    pub grid: &'a GridWorld<N>,
    #[derivative(Hash = "ignore", PartialEq = "ignore", Debug = "ignore")]
    pd: PhantomData<R>,
    pub curr_square: (usize, usize),
}

impl<'a, R, const N: usize> GridState<'a, R, N>
where
    R: SimpleReward,
{
    pub fn from_grid(grid: &'a GridWorld<N>, square: (usize, usize)) -> Self {
        Self {
            grid,
            curr_square: square,
            pd: PhantomData,
        }
    }

    #[inline]
    fn succ(&self, next_square: (usize, usize)) -> Self {
        Self {
            grid: self.grid,
            curr_square: next_square,
            pd: PhantomData,
        }
    }

    fn succ_reward(&self, next_square: (usize, usize), prob: f64) -> (Self, R) {
        let next_state = self.succ(next_square);
        let reward = R::from_prob_val(prob, self.grid[next_square].0);

        (next_state, reward)
    }
}

impl<'a, R, const N: usize> State<SimpleQ, R, SimpleQ> for GridState<'a, R, N>
where
    R: SimpleReward,
{
    type SuccRewardIter = GridSuccessors<'a, R, N>;

    type Action = GridAction;

    type ActionIter = AvailableActions;

    fn successors(&self, action: Self::Action) -> Self::SuccRewardIter {
        let curr = self.grid.borrow()[self.curr_square];
        match (curr.1, curr.2, action.next_coords(&self.curr_square, N)) {
            (chance @ RandomTransition::RealAction { prob, .. }, false, Some(succ)) => {
                let first = self.succ_reward(succ, 1.0 - prob);
                let second =
                    self.succ_reward(chance.false_move(&self.curr_square, N).unwrap(), prob);
                GridSuccessors::twice(first, second)
            }
            (RandomTransition::None, false, Some(succ)) => {
                GridSuccessors::once(self.succ_reward(succ, 1.0))
            }
            (_, true, _) | (_, false, None) => GridSuccessors::empty(),
        }
    }

    #[inline]
    fn actions(&self) -> Self::ActionIter {
        if !self.grid[self.curr_square].2 {
            GridAction::available_actions(self.curr_square, N)
        } else {
            AvailableActions::Empty
        }
    }
}

#[derive(Default)]
pub struct GridSuccessors<'a, R, const N: usize>
where
    R: SimpleReward,
{
    first: Option<(GridState<'a, R, N>, R)>,
    second: Option<(GridState<'a, R, N>, R)>,
}

impl<'a, R, const N: usize> GridSuccessors<'a, R, N>
where
    R: SimpleReward,
{
    fn empty() -> Self {
        Self {
            ..Default::default()
        }
    }
    fn once(succ: (GridState<'a, R, N>, R)) -> Self {
        Self {
            first: Some(succ),
            second: None,
        }
    }

    fn twice(succ: (GridState<'a, R, N>, R), succ2: (GridState<'a, R, N>, R)) -> Self {
        Self {
            first: Some(succ),
            second: Some(succ2),
        }
    }
}

impl<'a, R, const N: usize> Iterator for GridSuccessors<'a, R, N>
where
    R: SimpleReward,
{
    type Item = (GridState<'a, R, N>, R);

    fn next(&mut self) -> Option<Self::Item> {
        self.first.take().or_else(|| self.second.take())
    }
}

#[derive(Default, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum RandomTransition {
    #[default]
    None,
    RealAction {
        action: GridAction,
        prob: f64,
    },
}

impl RandomTransition {
    fn false_move(&self, coords: &(usize, usize), bounds: usize) -> Option<(usize, usize)> {
        if let Self::RealAction { action, .. } = self {
            action.next_coords(coords, bounds).or(Some(*coords))
        } else {
            None
        }
    }
}
