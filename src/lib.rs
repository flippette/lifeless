//!
//! A `no_std`-friendly library for implementing
//! [Conway's Game of Life](https://www.wikiwand.com/en/Conway's_Game_of_Life).
//!
//! See:
//! - [`Cell`] for the cells.
//! - [`Grid`] for the cell grid.
//! - [`Coord`] for the coordinates used in the cell grid.
//!

#![no_std]
#![forbid(unsafe_code)]

pub mod cell;
pub mod grid;
pub mod math;

pub use cell::Cell;
pub use grid::Grid;
pub use math::Coord;
