use core::num::NonZeroUsize;

use derive_more::{Add, Sub};

///
/// A vec2 type with [`usize`] components.
///
#[derive(Clone, Copy, PartialEq, Eq, Debug, Add, Sub)]
pub struct SVec2 {
    x: usize,
    y: usize,
}

///
/// Neighbors of a [`SVec2`].
///
/// Obtained by calling [`SVec2::neighbors()`].
///
#[derive(Clone, Copy, Debug)]
pub struct Neighbors {
    top_left: Option<SVec2>,
    top: Option<SVec2>,
    top_right: Option<SVec2>,
    right: Option<SVec2>,
    bottom_right: Option<SVec2>,
    bottom: Option<SVec2>,
    bottom_left: Option<SVec2>,
    left: Option<SVec2>,
}

///
/// [`Iterator`] over [`Neighbors`].
///
#[derive(Clone, Copy, Debug)]
pub struct NeighborsIter {
    neighbors: Neighbors,
    index: u8,
}

/// Shorthand for constructing a [`SVec2`], panicking on zero arguments.
pub const fn svec2(x: usize, y: usize) -> SVec2 {
    if let (Some(x), Some(y)) = (NonZeroUsize::new(x), NonZeroUsize::new(y)) {
        SVec2::new(x, y)
    } else {
        panic!("svec2() received zero-valued arguments!");
    }
}

impl SVec2 {
    ///
    /// Constructs a new [`SVec2`] with type-checked arguments.
    ///
    /// See [`svec2`] for a shorter (but panicking) version.
    ///
    #[inline]
    pub const fn new(x: NonZeroUsize, y: NonZeroUsize) -> Self {
        Self {
            x: x.get(),
            y: y.get(),
        }
    }

    ///
    /// Returns the neighbors of a position.
    ///
    #[rustfmt::skip]
    pub fn neighbors(&self, extents: Self) -> Neighbors {
        macro_rules! pos {
            (L) => { self.x == 1 };
            (R) => { self.x == extents.x };
            (T) => { self.y == 1 };
            (B) => { self.y == extents.y };
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
            top_left: if_opt!(!pos!(TL), self.up().left()),
            top: if_opt!(!pos!(T), self.up()),
            top_right: if_opt!(!pos!(TR), self.up().right()),
            right: if_opt!(!pos!(R), self.right()),
            bottom_right: if_opt!(!pos!(BR), self.down().right()),
            bottom: if_opt!(!pos!(B), self.down()),
            bottom_left: if_opt!(!pos!(BL), self.down().left()),
            left: if_opt!(!pos!(L), self.left()),
        }
    }
}

#[rustfmt::skip]
impl SVec2 {
    #[inline] pub fn x(&self) -> usize { self.x }
    #[inline] pub fn y(&self) -> usize { self.y }

    #[inline] pub fn up(&self) -> Self { Self { x: self.x , y: self.y - 1 } }
    #[inline] pub fn down(&self) -> Self { Self { x: self.x, y: self.y + 1 } }
    #[inline] pub fn left(&self) -> Self { Self { x: self.x - 1, y: self.y } }
    #[inline] pub fn right(&self) -> Self { Self { x: self.x + 1, y: self.y } }
}

impl Default for SVec2 {
    fn default() -> Self {
        svec2(1, 1)
    }
}

impl IntoIterator for Neighbors {
    type Item = SVec2;
    type IntoIter = NeighborsIter;

    fn into_iter(self) -> Self::IntoIter {
        NeighborsIter {
            neighbors: self,
            index: 0,
        }
    }
}

impl NeighborsIter {
    fn get_current(&self) -> Option<SVec2> {
        match self.index {
            0 => self.neighbors.top_left,
            1 => self.neighbors.top,
            2 => self.neighbors.top_right,
            3 => self.neighbors.right,
            4 => self.neighbors.bottom_right,
            5 => self.neighbors.bottom,
            6 => self.neighbors.bottom_left,
            7 => self.neighbors.left,
            _ => None,
        }
    }
}

impl Iterator for NeighborsIter {
    type Item = SVec2;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 8 {
            if let Some(pos) = self.get_current() {
                self.index += 1;
                return Some(pos);
            }
            self.index += 1;
        }

        None
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
        let extents = svec2(16, 16);

        let tl = svec2(1, 1).neighbors(extents);
        let tr = svec2(16, 1).neighbors(extents);
        let bl = svec2(1, 16).neighbors(extents);
        let br = extents.neighbors(extents);

        assert!(
            tl.top_left.is_none()
                && tl.top.is_none()
                && tl.top_right.is_none()
                && tl.right.is_some()
                && tl.bottom_right.is_some()
                && tl.bottom.is_some()
                && tl.bottom_left.is_none()
                && tl.left.is_none()
        );
        assert!(
            tr.top_left.is_none()
                && tr.top.is_none()
                && tr.top_right.is_none()
                && tr.right.is_none()
                && tr.bottom_right.is_none()
                && tr.bottom.is_some()
                && tr.bottom_left.is_some()
                && tr.left.is_some()
        );
        assert!(
            bl.top_left.is_none()
                && bl.top.is_some()
                && bl.top_right.is_some()
                && bl.right.is_some()
                && bl.bottom_right.is_none()
                && bl.bottom.is_none()
                && bl.bottom_left.is_none()
                && bl.left.is_none()
        );
        assert!(
            br.top_left.is_some()
                && br.top.is_some()
                && br.top_right.is_none()
                && br.right.is_none()
                && br.bottom_right.is_none()
                && br.bottom.is_none()
                && br.bottom_left.is_none()
                && br.left.is_some()
        );
    }

    #[test]
    fn neighbors_middle() {
        let neighbors = svec2(2, 2).neighbors(svec2(3, 3));

        assert!(
            neighbors.top_left.is_some()
                && neighbors.top.is_some()
                && neighbors.top_right.is_some()
                && neighbors.right.is_some()
                && neighbors.bottom_right.is_some()
                && neighbors.bottom.is_some()
                && neighbors.bottom_left.is_some()
                && neighbors.left.is_some()
        );
    }

    #[test]
    fn neighbors_iter_peri() {
        let mut tl = svec2(1, 1).neighbors(svec2(3, 3)).into_iter();
        let mut tr = svec2(3, 1).neighbors(svec2(3, 3)).into_iter();
        let mut bl = svec2(1, 3).neighbors(svec2(3, 3)).into_iter();
        let mut br = svec2(3, 3).neighbors(svec2(3, 3)).into_iter();

        assert_iter_next_eq!(tl, svec2(2, 1), svec2(2, 2), svec2(1, 2));
        assert_iter_next_eq!(tr, svec2(3, 2), svec2(2, 2), svec2(2, 1));
        assert_iter_next_eq!(bl, svec2(1, 2), svec2(2, 2), svec2(2, 3));
        assert_iter_next_eq!(br, svec2(2, 2), svec2(3, 2), svec2(2, 3));
    }

    #[test]
    fn neighbors_iter_middle() {
        let mut iter = svec2(2, 2).neighbors(svec2(3, 3)).into_iter();

        assert_iter_next_eq!(
            iter,
            svec2(1, 1),
            svec2(2, 1),
            svec2(3, 1),
            svec2(3, 2),
            svec2(3, 3),
            svec2(2, 3),
            svec2(1, 3),
            svec2(1, 2),
        );
    }
}
