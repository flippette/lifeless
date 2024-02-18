use core::{
    array,
    ops::{Index, IndexMut},
};

use crate::{cell::Cell, math::Coord};

///
/// The Game of Life cell grid.
///
/// [`Coord`]-based indices are in the range `(0..W, 0..H)`.
///
#[derive(Clone, Debug)]
pub struct Grid<const W: usize, const H: usize> {
    pub cells: [[Cell; W]; H],
    pub generation: u64,
}

impl<const W: usize, const H: usize> Grid<W, H> {
    /// Construct a new [`Coord`] with all [`Cell::Dead`] cells.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Dead; W]; H],
            generation: 0,
        }
    }

    ///
    /// Calculate the state of this cell in the next generation.
    ///
    /// Rules are in accordance to
    /// [the Wiki page](https://www.wikiwand.com/en/Conway's_Game_of_Life).
    ///
    #[must_use]
    pub fn state_next(&self, coord: Coord) -> Cell {
        match coord
            .neighbors(Coord(W, H))
            .filter(|&coord| self[coord] == Cell::Alive)
            .count()
        {
            0 | 1 | 4.. => Cell::Dead,
            2 => self[coord],
            3 => Cell::Alive,
        }
    }

    /// Calculates the next generation of this grid.
    #[must_use]
    pub fn step(&self) -> Self {
        Self {
            cells: array::from_fn(|y| array::from_fn(|x| self.state_next(Coord(x, y)))),
            generation: self.generation + 1,
        }
    }
}

impl<const W: usize, const H: usize> Default for Grid<W, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize, const H: usize> Index<Coord> for Grid<W, H> {
    type Output = Cell;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.cells[index.1][index.0]
    }
}

impl<const W: usize, const H: usize> IndexMut<Coord> for Grid<W, H> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.cells[index.1][index.0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn state_next() {
        let mut grid = Grid::<3, 3>::new();

        macro_rules! next {
            () => {
                grid.state_next(Coord(1, 1))
            };
        }

        // dies of loneliness
        grid[Coord(1, 1)] = Cell::Alive;
        assert_eq!(next!(), Cell::Dead);
        grid[Coord(0, 1)] = Cell::Alive;
        assert_eq!(next!(), Cell::Dead);

        // lives on
        grid[Coord(2, 2)] = Cell::Alive;
        assert_eq!(next!(), Cell::Alive);

        // births anew
        grid[Coord(1, 1)] = Cell::Dead;
        grid[Coord(0, 0)] = Cell::Alive;
        assert_eq!(next!(), Cell::Alive);

        // dies of overpopulation
        grid[Coord(0, 2)] = Cell::Alive;
        assert_eq!(next!(), Cell::Dead);
    }
}
