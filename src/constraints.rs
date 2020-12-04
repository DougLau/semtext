// constraints.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use std::ops::{Bound, RangeBounds};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints1 {
    /// Minimum bound (inclusive)
    minimum: u16,
    /// Maximum bound (exclusive)
    maximum: u16,
}

/// Widget layout constraints
///
/// These are minimum and maximum bounds for [Widget] dimensions, in cells.
/// This included column and row constraints.
///
/// They can be specified using range syntax.
///
/// [Widget]: trait.Widget.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
    /// Column constraints
    pub col: Constraints1,
    /// Vertical constraints
    pub row: Constraints1,
}

impl Default for Constraints1 {
    fn default() -> Self {
        Constraints1 {
            minimum: u16::MIN,
            maximum: u16::MAX,
        }
    }
}

impl Constraints1 {
    /// Create one dimensional constraints
    pub fn new<R>(bounds: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        let minimum = min_bound(bounds.start_bound());
        let maximum = max_bound(bounds.end_bound());
        Constraints1 { minimum, maximum }
    }

    /// Get the minimum bound (inclusive)
    pub fn minimum(self) -> u16 {
        self.minimum
    }

    /// Get the maximum bound (exclusive)
    pub fn maximum(self) -> u16 {
        self.maximum
    }

    /// Get the available amount to increase
    pub fn available(self) -> u16 {
        self.maximum - self.minimum
    }

    /// Increase minimum amount
    pub fn increase(&mut self) {
        self.minimum += 1;
        if self.minimum >= self.maximum {
            self.maximum = self.minimum + 1;
        }
    }

    /// Decrease maximum amount
    pub fn decrease(&mut self, amount: u16) {
        self.maximum = amount.max(self.minimum + 1);
    }
}

/// Get minimum from a bound
fn min_bound(bound: Bound<&u16>) -> u16 {
    match bound {
        Bound::Unbounded => u16::MIN,
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x + 1,
    }
}

/// Get maximum from a bound
fn max_bound(bound: Bound<&u16>) -> u16 {
    match bound {
        Bound::Unbounded => u16::MAX,
        Bound::Included(x) => *x + 1,
        Bound::Excluded(x) => *x,
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            col: Constraints1::default(),
            row: Constraints1::default(),
        }
    }
}

impl Constraints {
    /// Create new constraints
    fn new(col: Constraints1, row: Constraints1) -> Self {
        Constraints { col, row }
    }

    /// Adjust column constraints
    ///
    /// ```rust
    /// use semtext::Constraints;
    ///
    /// let c0 = Constraints::default().with_columns(..10);
    /// let c1 = Constraints::default().with_columns(2..);
    /// ```
    pub fn with_columns<R>(mut self, col: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.col = Constraints1::new(col);
        self
    }

    /// Adjust row constraints
    ///
    /// ```rust
    /// use semtext::Constraints;
    ///
    /// let c0 = Constraints::default().with_rows(1..8);
    /// let c1 = Constraints::default().with_rows(2..=4);
    /// ```
    pub fn with_rows<R>(mut self, row: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.row = Constraints1::new(row);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constraints() {
        let con = Constraints::default();
        assert_eq!(con.col.minimum, 0);
        assert_eq!(con.col.maximum, u16::MAX);
        assert_eq!(con.row.minimum, 0);
        assert_eq!(con.row.maximum, u16::MAX);
        let con = Constraints::default().with_columns(..5).with_rows(2..=2);
        assert_eq!(con.col.minimum, 0);
        assert_eq!(con.col.maximum, 5);
        assert_eq!(con.row.minimum, 2);
        assert_eq!(con.row.maximum, 3);
    }
}
