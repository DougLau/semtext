// spacer.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{AreaBound, Widget};
use std::ops::RangeBounds;

/// Spacer widget
#[derive(Default)]
pub struct Spacer {
    bounds: AreaBound,
}

impl Spacer {
    /// Adjust column spacing (bounds)
    ///
    /// ```rust
    /// use semtext::Spacer;
    ///
    /// let s0 = Spacer::default().with_columns(..10);
    /// let s1 = Spacer::default().with_columns(2..);
    /// ```
    pub fn with_columns<R>(mut self, col: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.bounds = self.bounds.with_columns(col);
        self
    }

    /// Adjust row spacing (bounds)
    ///
    /// ```rust
    /// use semtext::Spacer;
    ///
    /// let s0 = Spacer::default().with_rows(1..8);
    /// let s1 = Spacer::default().with_rows(2..=4);
    /// ```
    pub fn with_rows<R>(mut self, row: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.bounds = self.bounds.with_rows(row);
        self
    }
}

impl Widget for Spacer {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        self.bounds
    }
}
