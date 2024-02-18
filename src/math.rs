use core::{
    array,
    ops::{Add, Sub},
};

///
/// A 2D coordinate.
///
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Coord(pub usize, pub usize);

///
/// Neighbors of a [`Coord`].
///
/// Obtained by calling [`Coord::neighbors()`].
///
#[derive(Clone, Debug)]
pub struct Neighbors {
    inner: array::IntoIter<Option<Coord>, 8>,
}

impl Coord {
    ///
    /// Returns the neighbors of a [`Coord`].
    ///
    #[rustfmt::skip]
    #[must_use]
    pub fn neighbors(&self, extents: Self) -> Neighbors {
        macro_rules! pos {
            (L) => { self.0 == 0 };
            (R) => { self.0 >= extents.0 - 1 };
            (T) => { self.1 == 0 };
            (B) => { self.1 >= extents.1 - 1 };
            (TL) => { pos!(T) || pos!(L) };
            (TR) => { pos!(T) || pos!(R) };
            (BL) => { pos!(B) || pos!(L) };
            (BR) => { pos!(B) || pos!(R) };
        }

        macro_rules! if_opt {
            ($pred:expr, $val:expr) => {{
                if $pred { Some($val) } else { None }
            }};
        }

        Neighbors {
            inner: [
                if_opt!(!pos!(TL), self.up().left()),
                if_opt!(!pos!(T), self.up()),
                if_opt!(!pos!(TR), self.up().right()),
                if_opt!(!pos!(R), self.right()),
                if_opt!(!pos!(BR), self.down().right()),
                if_opt!(!pos!(B), self.down()),
                if_opt!(!pos!(BL), self.down().left()),
                if_opt!(!pos!(L), self.left()),
            ].into_iter()
        }
    }
}

#[rustfmt::skip]
impl Coord {
    #[inline] #[must_use] pub fn x(&self) -> usize { self.0 }
    #[inline] #[must_use] pub fn y(&self) -> usize { self.1 }

    #[inline] #[must_use] pub fn up(&self) -> Self { Self(self.0, self.1 - 1) }
    #[inline] #[must_use] pub fn down(&self) -> Self { Self(self.0, self.1 + 1) }
    #[inline] #[must_use] pub fn left(&self) -> Self { Self(self.0 - 1, self.1) }
    #[inline] #[must_use] pub fn right(&self) -> Self { Self(self.0 + 1, self.1) }
}

impl Add<Self> for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Self> for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Iterator for Neighbors {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(coord) = self.inner.next()? {
                break Some(coord);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_iter_next_eq {
        ($iter:expr, $($e:expr),+ $(,)?) => {{
            $(assert_eq!($iter.next(), Some($e));)+
        }}
    }

    #[test]
    fn neighbors_peri() {
        let extents = Coord(3, 3);

        let mut tl = Coord(0, 0).neighbors(extents);
        let mut tr = Coord(2, 0).neighbors(extents);
        let mut bl = Coord(0, 2).neighbors(extents);
        let mut br = Coord(2, 2).neighbors(extents);

        assert_iter_next_eq!(tl, Coord(1, 0), Coord(1, 1), Coord(0, 1));
        assert_iter_next_eq!(tr, Coord(2, 1), Coord(1, 1), Coord(1, 0));
        assert_iter_next_eq!(bl, Coord(0, 1), Coord(1, 1), Coord(1, 2));
        assert_iter_next_eq!(br, Coord(1, 1), Coord(2, 1), Coord(1, 2));
    }

    #[test]
    fn neighbors_iter_middle() {
        let mut iter = Coord(1, 1).neighbors(Coord(3, 3));

        assert_iter_next_eq!(
            iter,
            Coord(0, 0),
            Coord(1, 0),
            Coord(2, 0),
            Coord(2, 1),
            Coord(2, 2),
            Coord(1, 2),
            Coord(0, 2),
            Coord(0, 1),
        );
    }
}
