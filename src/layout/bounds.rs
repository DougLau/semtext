// bounds.rs
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use std::ops::{Add, Bound, RangeBounds};

/// Bounds on columns or rows
///
/// This restricts minimum and maximum allowed length.  The minimum bound is
/// treated as a "hard" restriction, while the maximum is "soft".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LengthBound {
    /// Minimum bound (inclusive)
    ///
    /// The `Unbounded` case uses the `MIN` value
    minimum: u16,

    /// Maximum bound (exclusive)
    ///
    /// The `Unbounded` case uses the `MAX` value
    maximum: u16,
}

impl Default for LengthBound {
    fn default() -> Self {
        LengthBound {
            minimum: u16::MIN,
            maximum: u16::MAX,
        }
    }
}

impl Add for LengthBound {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let minimum = self.minimum.saturating_add(rhs.minimum);
        let maximum = if self.maximum < u16::MAX && rhs.maximum < u16::MAX {
            self.maximum.saturating_add(rhs.maximum)
        } else {
            self.maximum.min(rhs.maximum)
        };
        let maximum = maximum.max(minimum);
        LengthBound { minimum, maximum }
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
        self.maximum = self.maximum.max(self.minimum);
    }

    /// Decrease maximum bound
    pub fn decrease(&mut self, amount: u16) {
        self.maximum = amount.max(self.minimum);
    }
}

/// Get minimum from a `Bound`
fn min_bound(bound: Bound<&u16>) -> u16 {
    match bound {
        Bound::Unbounded => u16::MIN,
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x + 1,
    }
}

/// Get maximum from a `Bound`
fn max_bound(bound: Bound<&u16>) -> u16 {
    match bound {
        Bound::Unbounded => u16::MAX,
        Bound::Included(x) => *x + 1,
        Bound::Excluded(x) => *x,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds() {
        let bnd = LengthBound::default();
        assert_eq!(bnd.minimum, 0);
        assert_eq!(bnd.maximum, u16::MAX);
        let bnd = LengthBound::new(..5);
        assert_eq!(bnd.minimum, 0);
        assert_eq!(bnd.maximum, 5);
        let bnd = LengthBound::new(2..=2);
        assert_eq!(bnd.minimum, 2);
        assert_eq!(bnd.maximum, 3);
    }
}
