use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::rewards::*;
use crate::simple_solvers::{
    environments::{convergence::StandardConvergenceMeasure, *},
    simple_solver, State,
};

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
    const N: usize = 15;

    let grid = GridWorld::<N>::default();
    let state = GridState::from_grid(&grid, (1, 1));

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

    assert_eq!(GridState::from_grid(&grid, (0, 1),), next.0);
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
    assert_eq!(terminal.0, GridState::from_grid(&grid, (0, 0)));
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
    const N: usize = 15;

    let grid = GridWorld::<N>::default();
    let start_state: GridState<'_, NoDiscountReward, N> = GridState::from_grid(&grid, (14, 14));
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

    // Use `fold` instead of `all` so we see all the examples that are wrong instead of just
    // the first one
    let correct = results
        .into_iter()
        .flat_map(|(s, inner)| inner.into_iter().map(move |(a, q)| (s, a, q)))
        .fold(true, |acc, (s, a, q)| {
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
            acc && outcome
        });

    assert!(correct);
}

#[test]
fn test_discounted_solver() {
    const X: i64 = 7;
    const Y: i64 = 7;
    const DISCOUNT: i64 = 80;
    const REWARD: i64 = 15;
    const N: usize = 15;

    let mut grid = GridWorld::<N>::default();
    grid[(0, 0)] = (0, RandomTransition::None, false);
    grid[(0, 1)] = (0, RandomTransition::None, false);
    grid[(X as usize, Y as usize)] = (REWARD, RandomTransition::None, true);
    let start_state: GridState<'_, DiscountedReward<80>, N> = GridState::from_grid(&grid, (14, 14));

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

    // Use `fold` instead of `all` so we see all the examples that are wrong instead of just
    // the first one
    let correct = results
        .into_iter()
        .flat_map(|(s, inner)| inner.into_iter().map(move |(a, q)| (s, a, q)))
        .fold(true, |acc, (s, a, q)| {
            let appropriate_q = if s.curr_square == (X as usize, Y as usize) {
                0.0
            } else if let Some(next) = a.next_coords(&s.curr_square, 15) {
                let manhattan_distance = (next.0 as i64 - X).abs() + (next.1 as i64 - Y).abs();

                (DISCOUNT as f64 / 100.).powi(manhattan_distance as i32) * REWARD as f64
            } else {
                0.0
            };

            acc && if (appropriate_q - q.0).abs() > 0.01 {
                println!(
                    "Wrong! State: {:?}, Action: {:?}, Q-val: {:?}, Expected: {}",
                    s, a, q, appropriate_q
                );
                false
            } else {
                true
            }
        });

    assert!(correct);
}
