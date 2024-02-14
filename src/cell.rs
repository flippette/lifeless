use core::ops::Not;

///
/// A cell on a grid.
///
#[derive(Clone, Copy, Debug, Default)]
#[repr(u8)]
pub enum Cell {
    Alive = 1,
    #[default]
    Dead = 0,
}

#[rustfmt::skip]
impl Cell {
    #[inline] pub fn is_alive(&self) -> bool { matches!(self, Self::Alive) }
    #[inline] pub fn is_dead(&self) -> bool { !self.is_alive() }
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
