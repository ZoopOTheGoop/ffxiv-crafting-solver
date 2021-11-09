use strum::{EnumIter, IntoEnumIterator};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash, EnumIter)]
pub enum GridAction {
    North,
    South,
    East,
    West,
}

impl GridAction {
    pub fn next_coords(&self, coords: &(usize, usize), bounds: usize) -> Option<(usize, usize)> {
        match (self, coords) {
            (GridAction::North, (x, y)) if y > &0 => Some((*x, y - 1)),
            (GridAction::South, (x, y)) if y + 1 < bounds => Some((*x, y + 1)),
            (GridAction::East, (x, y)) if x + 1 < bounds => Some((x + 1, *y)),
            (GridAction::West, (x, y)) if x > &0 => Some((x - 1, *y)),
            _ => None,
        }
    }

    pub fn available_actions(coords: (usize, usize), bounds: usize) -> AvailableActions {
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
