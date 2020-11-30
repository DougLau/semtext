// constraints.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use std::ops::{Bound, RangeBounds};

/// Widget dimension constraints
///
/// These are minimum and maximum bounds for widget size, in cells.
/// Width is constrained by horizontal bounds, height by vertical.
///
/// They can be specified using range syntax.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
    /// Horizontal start bound
    horiz_start: Bound<u16>,
    /// Horizontal end bound
    horiz_end: Bound<u16>,
    /// Vertical start bound
    vert_start: Bound<u16>,
    /// Vertical end bound
    vert_end: Bound<u16>,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            horiz_start: Bound::Unbounded,
            horiz_end: Bound::Unbounded,
            vert_start: Bound::Unbounded,
            vert_end: Bound::Unbounded,
        }
    }
}

/// Get cloned range bound
fn bound_cloned<T: Clone>(bound: Bound<&T>) -> Bound<T> {
    match bound {
        Bound::Unbounded => Bound::Unbounded,
        Bound::Included(x) => Bound::Included(x.clone()),
        Bound::Excluded(x) => Bound::Excluded(x.clone()),
    }
}

impl Constraints {
    /// Adjust horizontal constraints
    ///
    /// ```rust
    /// use semtext::Constraints;
    ///
    /// let c0 = Constraints::default().with_horizontal(..10);
    /// let c1 = Constraints::default().with_horizontal(2..);
    /// ```
    pub fn with_horizontal<R>(mut self, horiz: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.horiz_start = bound_cloned(horiz.start_bound());
        self.horiz_end = bound_cloned(horiz.end_bound());
        self
    }

    /// Adjust vertical constraints
    ///
    /// ```rust
    /// use semtext::Constraints;
    ///
    /// let c0 = Constraints::default().with_vertical(1..8);
    /// let c1 = Constraints::default().with_vertical(2..=4);
    /// ```
    pub fn with_vertical<R>(mut self, vert: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.vert_start = bound_cloned(vert.start_bound());
        self.vert_end = bound_cloned(vert.end_bound());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constraints() {
        let con = Constraints::default();
        assert_eq!(con.horiz_start, Bound::Unbounded);
        assert_eq!(con.horiz_end, Bound::Unbounded);
        assert_eq!(con.vert_start, Bound::Unbounded);
        assert_eq!(con.vert_end, Bound::Unbounded);
        let con = Constraints::default()
            .with_horizontal(..5)
            .with_vertical(2..=2);
        assert_eq!(con.horiz_start, Bound::Unbounded);
        assert_eq!(con.horiz_end, Bound::Excluded(5));
        assert_eq!(con.vert_start, Bound::Included(2));
        assert_eq!(con.vert_end, Bound::Included(2));
    }
}
