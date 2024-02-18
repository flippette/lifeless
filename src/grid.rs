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
    cells: [[Cell; W]; H],
    generation: u64,
}

impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn new() -> Self {
        Self {
            cells: array::from_fn(|_| array::from_fn(|_| Cell::Dead)),
            generation: 0,
        }
    }

    ///
    /// Calculate the state of this cell in the next generation.
    ///
    /// Rules are in accordance to
    /// [the Wiki page](https://www.wikiwand.com/en/Conway's_Game_of_Life).
    ///
    pub fn state_next(&self, coord: Coord) -> Cell {
        let live_neighbors = coord
            .neighbors(Coord(W, H))
            .filter(|&pos| self[pos].is_alive())
            .count();

        match live_neighbors {
            0 | 1 | 4.. => Cell::Dead,
            2 => self[coord],
            3 => Cell::Alive,
        }
    }

    /// Calculates the next generation of this grid.
    pub fn step(&self) -> Self {
        Self {
            cells: array::from_fn(|y| array::from_fn(|x| self.state_next(Coord(x, y)))),
            generation: self.generation() + 1,
        }
    }
}

#[rustfmt::skip]
impl<const W: usize, const H: usize> Grid<W, H> {
    #[inline] pub fn columns(&self) -> usize { self.cells[0].len() }
    #[inline] pub fn rows(&self) -> usize { self.cells.len() }
    #[inline] pub fn cells(&self) -> &[[Cell; W]; H] { &self.cells }
    #[inline] pub fn generation(&self) -> &u64 { &self.generation }
    #[inline] pub fn set(&mut self, pos: Coord, state: Cell) { self[pos] = state }
    #[inline] pub fn toggle(&mut self, pos: Coord) { self.set(pos, !self[pos]) }
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
