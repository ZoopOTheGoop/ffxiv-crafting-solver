use std::{iter, slice};

use super::*;
use crate::prelude::*;

struct OneDGrid(Vec<()>);

struct GridLocation(usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GridReward(Option<usize>);

impl Composable for GridReward {
    fn compose(&self, other: &Self) -> Self {
        self.0.zip(other).map(|(a, b)| a + b).or(self.0).or(other)
    }
}

impl Bellman for GridReward {
    fn update(&self, other: &Self) -> Self {
        self.compose(other)
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum GridAction {
    Left,
    Right,
}

impl Action for GridAction {}

impl ActionSet<GridLocation, Self, GridReward> for GridAction {
    type Iter = ();

    fn filter(&self, curr_state: &GridLocation) -> Self::Iter {
        if curr_state.0 == 0 {
            [Self::Right].iter().copied()
        } else if curr_state.0 == 10 {
            [Self::Left].iter().copied()
        } else if curr_state == 0 {
            [].iter().copied()
        } else {
            [Self::Left, Self::Right].iter().copied()
        }
    }

    fn iter(&self) -> Self::Iter {
        [Self::Left, Self::Right]
    }

    fn count(&self) -> usize {
        2
    }
}

impl State<GridReward, GridAction> for GridLocation {
    type Iter = GridStateIterator;

    fn successors(&self, action: GridAction) -> Self::Iter {
        match action {
            Action::Left => {
                if self.0 > 0 && self.0 != 5 {
                    GridStateIterator(
                        self.0 - 1,
                        if self.0 - 1 == 5 {
                            GridReward::default
                        } else {
                            GridReward(Some(10))
                        },
                    )
                }
            }
            Action::Right => {
                if self.0 < 10 && self.0 != 5 {
                    GridStateIterator(
                        Some(self.0 + 1),
                        if self.0 + 1 == 5 {
                            GridReward::default
                        } else {
                            GridReward(Some(10))
                        },
                    )
                }
            }
            5 => iter::empty(),
            _ => panic!("Out of bounds"),
        }
    }
}

struct GridStateIterator(Option<(GridLocation, GridReward)>);

impl Iterator for GridStateIterator {
    type Item = (GridLocation, GridReward);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
    }
}
