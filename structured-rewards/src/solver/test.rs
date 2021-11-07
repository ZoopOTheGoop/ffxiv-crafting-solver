#![cfg(test)]

use std::{
    borrow::Borrow,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use super::*;

use derivative::Derivative;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct NoDiscountReward(f64, i64);

pub trait SimpleReward: Sized + Compose<SimpleQ, SimpleQ> + Default + std::fmt::Debug {
    fn from_prob_val(prob: f64, val: i64) -> Self;
}

impl SimpleReward for NoDiscountReward {
    fn from_prob_val(prob: f64, val: i64) -> Self {
        Self(prob, val)
    }
}

impl Compose<SimpleQ, SimpleQ> for NoDiscountReward {
    fn compose(&self, other: &SimpleQ) -> SimpleQ {
        SimpleQ(self.0 * (self.1 as f64 + other.0))
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DiscountedReward<const D: i64>(f64, i64);

impl<const D: i64> SimpleReward for DiscountedReward<D> {
    fn from_prob_val(prob: f64, val: i64) -> Self {
        Self(prob, val)
    }
}

impl<const D: i64> Compose<SimpleQ, SimpleQ> for DiscountedReward<D> {
    fn compose(&self, other: &SimpleQ) -> SimpleQ {
        SimpleQ(self.0 * (self.1 as f64 + (D as f64 / 100.) * other.0))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct SimpleQ(f64);

impl QVal for SimpleQ {}

impl Bellman for SimpleQ {
    fn update(&self, other: &Self) -> Self {
        SimpleQ(self.0 + other.0)
    }

    fn partial_update(&self, other: &Self) -> Self {
        self.update(other)
    }

    fn reweight(&self) -> Self {
        *self
    }
}

impl SemanticOrd for SimpleQ {
    fn sem_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl SemanticEq for SimpleQ {
    fn sem_eq(&self, other: &Self) -> bool {
        debug_assert!(!self.0.is_nan());
        debug_assert!(!other.0.is_nan());
        self.0 == other.0
    }
}

#[derive(Derivative, PartialEq, PartialOrd, Debug, Copy, Clone)]
#[derivative(Default)]
pub enum RandomTransition {
    #[derivative(Default)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash, EnumIter)]
pub enum GridAction {
    North,
    South,
    East,
    West,
}

impl GridAction {
    fn next_coords(&self, coords: &(usize, usize), bounds: usize) -> Option<(usize, usize)> {
        match (self, coords) {
            (GridAction::North, (x, y)) if y > &0 => Some((*x, y - 1)),
            (GridAction::South, (x, y)) if y + 1 < bounds => Some((*x, y + 1)),
            (GridAction::East, (x, y)) if x + 1 < bounds => Some((x + 1, *y)),
            (GridAction::West, (x, y)) if x > &0 => Some((x - 1, *y)),
            _ => None,
        }
    }

    fn available_actions(coords: (usize, usize), bounds: usize) -> AvailableActions {
        AvailableActions::Iter {
            inner: Self::iter(),
            coords,
            bounds,
        }
    }
}

pub enum AvailableActions {
    Empty,
    Iter {
        inner: GridActionIter,
        coords: (usize, usize),
        bounds: usize,
    },
}

impl Iterator for AvailableActions {
    type Item = GridAction;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::Iter {
                inner,
                coords,
                bounds,
            } => inner.find(|v| v.next_coords(coords, *bounds).is_some()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GridWorld<const N: usize> {
    grid: [[(i64, RandomTransition); N]; N],
}

impl<const N: usize> Default for GridWorld<N> {
    fn default() -> Self {
        let mut me = GridWorld {
            grid: [[Default::default(); N]; N],
        };
        me[(0, 0)] = (15, RandomTransition::None);
        me[(0, 1)] = (
            0,
            RandomTransition::RealAction {
                action: GridAction::West,
                prob: 0.2,
            },
        );
        me
    }
}

impl<const N: usize> Index<(usize, usize)> for GridWorld<N> {
    type Output = (i64, RandomTransition);

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
    grid: &'a GridWorld<N>,
    #[derivative(Hash = "ignore", PartialEq = "ignore", Debug = "ignore")]
    pd: PhantomData<R>,
    curr_square: (usize, usize),
}

impl<'a, R, const N: usize> GridState<'a, R, N>
where
    R: SimpleReward,
{
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
        match (curr.0, curr.1, action.next_coords(&self.curr_square, N)) {
            (0, chance @ RandomTransition::RealAction { prob, .. }, Some(succ)) => {
                let first = self.succ_reward(succ, 1.0 - prob);
                let second =
                    self.succ_reward(chance.false_move(&self.curr_square, N).unwrap(), prob);
                GridSuccessors::twice(first, second)
            }
            (0, RandomTransition::None, Some(succ)) => {
                GridSuccessors::once(self.succ_reward(succ, 1.0))
            }
            _ => GridSuccessors::empty(),
        }
    }

    #[inline]
    fn actions(&self) -> Self::ActionIter {
        if self.grid[self.curr_square].0 == 0 {
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

#[test]
fn test_west() {
    let west = GridAction::West;

    let next = west.next_coords(&(5, 5), 10);
    assert_eq!(next, Some((4, 5)));

    let next = west.next_coords(&(5, 5), 5);
    assert_eq!(next, Some((4, 5)));

    let next = west.next_coords(&(0, 5), 10);
    assert_eq!(next, None);
}

#[test]
fn test_east() {
    let east = GridAction::East;

    let next = east.next_coords(&(5, 5), 10);
    assert_eq!(next, Some((6, 5)));

    let next = east.next_coords(&(5, 5), 5);
    assert_eq!(next, None);

    let next = east.next_coords(&(0, 5), 10);
    assert_eq!(next, Some((1, 5)));

    let next = east.next_coords(&(0, 0), 10);
    assert_eq!(next, Some((1, 0)));
}

#[test]
fn test_north() {
    let north = GridAction::North;

    let next = north.next_coords(&(5, 5), 10);
    assert_eq!(next, Some((5, 4)));

    let next = north.next_coords(&(5, 5), 5);
    assert_eq!(next, Some((5, 4)));

    let next = north.next_coords(&(5, 0), 10);
    assert_eq!(next, None);

    let next = north.next_coords(&(0, 0), 10);
    assert_eq!(next, None);
}

#[test]
fn test_south() {
    let south = GridAction::South;

    let next = south.next_coords(&(5, 5), 10);
    assert_eq!(next, Some((5, 6)));

    let next = south.next_coords(&(5, 5), 5);
    assert_eq!(next, None);

    let next = south.next_coords(&(5, 0), 10);
    assert_eq!(next, Some((5, 1)));

    let next = south.next_coords(&(0, 0), 10);
    assert_eq!(next, Some((0, 1)));
}

#[test]
fn test_gridworld() {
    let grid = GridWorld::<15>::default();
    let state = GridState {
        grid: &grid,
        curr_square: (1, 1),
        pd: PhantomData,
    };

    assert_eq!(
        state.actions().collect::<Vec<_>>(),
        vec![
            GridAction::North,
            GridAction::South,
            GridAction::East,
            GridAction::West
        ]
    );

    let mut succ = state.successors(GridAction::West);
    let next = succ.next();
    assert!(next.is_some());
    let next = next.unwrap();
    assert!(succ.next().is_none());

    assert_eq!(
        GridState {
            grid: &grid,
            curr_square: (0, 1),
            pd: PhantomData,
        },
        next.0
    );
    assert_eq!(NoDiscountReward(1.0, 0), next.1);

    let state = next.0;
    assert_eq!(
        state.actions().collect::<Vec<_>>(),
        vec![GridAction::North, GridAction::South, GridAction::East]
    );

    assert_eq!(
        state.successors(GridAction::West).collect::<Vec<_>>(),
        vec![]
    );

    let mut succ = state.successors(GridAction::North);
    let terminal = succ.next();
    assert!(terminal.is_some());
    let terminal = terminal.unwrap();
    assert_eq!(
        terminal.0,
        GridState {
            grid: &grid,
            curr_square: (0, 0),
            pd: PhantomData,
        }
    );
    assert_eq!(terminal.1, NoDiscountReward(1.0 - 0.2, 15));

    assert_eq!(terminal.0.actions().collect::<Vec<_>>(), vec![]);

    let same = succ.next();
    assert!(same.is_some());
    let same = same.unwrap();
    assert_eq!(same.0, state);
    assert_eq!(same.1, NoDiscountReward(0.2, 0));

    assert!(succ.next().is_none());
}

#[test]
fn test_minimal_solver() {
    let grid = GridWorld::<15>::default();
    let start_state = GridState {
        grid: &grid,
        curr_square: (14, 14),
        pd: PhantomData::<NoDiscountReward>,
    };
    let results = simple_solver(start_state, StandardConvergenceMeasure);

    let mut grid = GridAction::iter()
        .map(|a| (a, [[0.; 15]; 15]))
        .collect::<HashMap<_, _>>();

    for (s, map) in results.iter() {
        for (a, q) in map.iter() {
            grid.get_mut(a).unwrap()[s.curr_square.1][s.curr_square.0] = q.0;
        }
    }

    for (a, qs) in grid.iter() {
        println!("{:?}:", a);
        for row in qs {
            println!("\t{:?}", row);
        }
    }

    let correct = results
        .into_iter()
        .flat_map(|(s, inner)| inner.into_iter().map(move |(a, q)| (s, a, q)))
        .map(|(s, a, q)| {
            let outcome = if s.curr_square == (1, 0) && a == GridAction::North {
                (q.0 - (0.2 * 15.)).abs() < 0.04
            } else if s.curr_square == (0, 0) // Terminal
                || a.next_coords(&s.curr_square, 15).is_none()
            // On the edge
            {
                q.0 == 0.0
            } else {
                (q.0 - 15.).abs() < 0.04
            };
            if !outcome {
                println!("Wrong! State: {:?}, Action: {:?}, Q-val: {:?}", s, a, q);
            }
            outcome
        })
        .collect::<Vec<_>>() // So we can see all errors and not just the first
        .into_iter()
        .all(|v| v);

    assert!(correct);
}

#[test]
fn test_discounted_solver() {
    const X: i64 = 7;
    const Y: i64 = 7;
    const DISCOUNT: i64 = 80;
    const REWARD: i64 = 15;

    let mut grid = GridWorld::<15>::default();
    grid[(0, 0)] = (0, RandomTransition::None);
    grid[(0, 1)] = (0, RandomTransition::None);
    grid[(X as usize, Y as usize)] = (REWARD, RandomTransition::None);
    let start_state = GridState {
        grid: &grid,
        curr_square: (14, 14),
        pd: PhantomData::<DiscountedReward<DISCOUNT>>,
    };
    let results = simple_solver(start_state, StandardConvergenceMeasure);

    /* prints (got, expected) for debugging */
    let mut grid = GridAction::iter()
        .map(|a| (a, [[(0., 0.); 15]; 15]))
        .collect::<HashMap<_, _>>();

    for (s, map) in results.iter() {
        for (a, q) in map.iter() {
            let next = a
                .next_coords(&s.curr_square, 15)
                .unwrap_or((X as usize, Y as usize));
            let manhattan_distance = (next.0 as i64 - X).abs() + (next.1 as i64 - Y).abs();
            let expected = (DISCOUNT as f64 / 100.).powi(manhattan_distance as i32) * REWARD as f64;
            grid.get_mut(a).unwrap()[s.curr_square.1][s.curr_square.0] = (q.0, expected);
        }
    }

    for (a, qs) in grid.iter() {
        println!("{:?}:", a);
        for row in qs {
            println!("\t{:.2?}", row);
        }
    }

    let correct = results
        .into_iter()
        .flat_map(|(s, inner)| inner.into_iter().map(move |(a, q)| (s, a, q)))
        .map(|(s, a, q)| {
            let appropriate_q = if s.curr_square == (X as usize, Y as usize) {
                0.0
            } else if let Some(next) = a.next_coords(&s.curr_square, 15) {
                let manhattan_distance = (next.0 as i64 - X).abs() + (next.1 as i64 - Y).abs();

                (DISCOUNT as f64 / 100.).powi(manhattan_distance as i32) * REWARD as f64
            } else {
                0.0
            };

            if (appropriate_q - q.0).abs() > 0.01 {
                println!(
                    "Wrong! State: {:?}, Action: {:?}, Q-val: {:?}, Expected: {}",
                    s, a, q, appropriate_q
                );
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>() // So we can see all errors and not just the first
        .into_iter()
        .all(|v| v);

    assert!(correct);
}
