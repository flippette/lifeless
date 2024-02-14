use core::{
    iter,
    ops::{Index, IndexMut},
};

use heapless::Vec;

use crate::{
    cell::Cell,
    math::{svec2, SVec2},
};

///
/// The Game of Life cell grid.
///
/// [`SVec2`]-based indices start from `(1, 1)` to `(W, H)`.
///
#[derive(Clone, Debug)]
pub struct Grid<const W: usize, const H: usize> {
    cells: Vec<Vec<Cell, W>, H>,
    generation: u64,
}

impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn new() -> Self {
        Self {
            cells: Vec::from_iter(
                iter::repeat(Vec::from_iter(iter::repeat(Cell::Dead).take(W))).take(H),
            ),
            generation: 0,
        }
    }

    ///
    /// Calculate the state of this cell in the next generation.
    ///
    /// Rules are in accordance to
    /// [the Wiki page](https://www.wikiwand.com/en/Conway's_Game_of_Life).
    ///
    pub fn state_next(&self, pos: SVec2) -> Cell {
        let live_neighbors = pos
            .neighbors(svec2(W, H))
            .into_iter()
            .filter(|&pos| self[pos].is_alive())
            .count();

        match live_neighbors {
            0 | 1 | 4.. => Cell::Dead,
            2 => self[pos],
            3 => Cell::Alive,
        }
    }

    /// Calculates the next generation of this grid.
    pub fn step(&self) -> Self {
        Self {
            cells: Vec::from_iter(
                (1..=H).map(|y| Vec::from_iter((1..=W).map(|x| self.state_next(svec2(x, y))))),
            ),
            generation: self.generation() + 1,
        }
    }
}

#[rustfmt::skip]
impl<const W: usize, const H: usize> Grid<W, H> {
    #[inline] pub fn cells(&self) -> &Vec<Vec<Cell, W>, H> { &self.cells }
    #[inline] pub fn generation(&self) -> &u64 { &self.generation }
    #[inline] pub fn set(&mut self, pos: SVec2, state: Cell) { self[pos] = state }
    #[inline] pub fn toggle(&mut self, pos: SVec2) { self.set(pos, !self[pos]) }
}

impl<const W: usize, const H: usize> Default for Grid<W, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize, const H: usize> Index<SVec2> for Grid<W, H> {
    type Output = Cell;

    fn index(&self, index: SVec2) -> &Self::Output {
        &self.cells[index.y() - 1][index.x() - 1]
    }
}

impl<const W: usize, const H: usize> IndexMut<SVec2> for Grid<W, H> {
    fn index_mut(&mut self, index: SVec2) -> &mut Self::Output {
        &mut self.cells[index.y() - 1][index.x() - 1]
    }
}
