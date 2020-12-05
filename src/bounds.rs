// bounds.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use std::ops::{Bound, RangeBounds};

/// Bounds on columns or rows
///
/// This restricts minimum and maximum allowed length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LengthBound {
    /// Minimum bound (inclusive)
    minimum: u16,
    /// Maximum bound (exclusive)
    maximum: u16,
}

/// Bounds restricting cell area
///
/// This includes column and row length bounds for [Widget]s, in cells.
/// They can be specified using range syntax.
///
/// ### Example
///
/// ```rust
/// use semtext::AreaBound;
///
/// let b = AreaBound::default().with_columns(6..=10).with_rows(1..);
/// ```
/// [Widget]: trait.Widget.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AreaBound {
    /// Cell column bounds
    pub(crate) col: LengthBound,
    /// Cell row bounds
    pub(crate) row: LengthBound,
}

impl Default for LengthBound {
    fn default() -> Self {
        LengthBound {
            minimum: u16::MIN,
            maximum: u16::MAX,
        }
    }
}

impl LengthBound {
    /// Create a new length bound
    pub fn new<R>(bounds: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        let minimum = min_bound(bounds.start_bound());
        let maximum = max_bound(bounds.end_bound());
        LengthBound { minimum, maximum }
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

    /// Increase minimum bound
    pub fn increase(&mut self, amount: u16) {
        self.minimum += amount;
        if self.minimum >= self.maximum {
            self.maximum = self.minimum + 1;
        }
    }

    /// Decrease maximum bound
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

impl Default for AreaBound {
    fn default() -> Self {
        AreaBound {
            col: LengthBound::default(),
            row: LengthBound::default(),
        }
    }
}

impl AreaBound {
    /// Adjust column bounds
    ///
    /// ### Example
    ///
    /// ```rust
    /// use semtext::AreaBound;
    ///
    /// let b0 = AreaBound::default().with_columns(..10);
    /// let b1 = AreaBound::default().with_columns(2..);
    /// ```
    pub fn with_columns<R>(mut self, col: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.col = LengthBound::new(col);
        self
    }

    /// Adjust row bounds
    ///
    /// ### Example
    ///
    /// ```rust
    /// use semtext::AreaBound;
    ///
    /// let b0 = AreaBound::default().with_rows(1..8);
    /// let b1 = AreaBound::default().with_rows(2..=4);
    /// ```
    pub fn with_rows<R>(mut self, row: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.row = LengthBound::new(row);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds() {
        let bnd = AreaBound::default();
        assert_eq!(bnd.col.minimum, 0);
        assert_eq!(bnd.col.maximum, u16::MAX);
        assert_eq!(bnd.row.minimum, 0);
        assert_eq!(bnd.row.maximum, u16::MAX);
        let bnd = AreaBound::default().with_columns(..5).with_rows(2..=2);
        assert_eq!(bnd.col.minimum, 0);
        assert_eq!(bnd.col.maximum, 5);
        assert_eq!(bnd.row.minimum, 2);
        assert_eq!(bnd.row.maximum, 3);
    }
}
