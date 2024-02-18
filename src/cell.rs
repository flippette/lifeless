use core::ops::Not;

///
/// A cell on a grid.
///
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum Cell {
    Alive = 1,
    #[default]
    Dead = 0,
}

impl Not for Cell {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Alive => Self::Dead,
            Self::Dead => Self::Alive,
        }
    }
}
