// spacer.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Constraints, Widget};
use std::ops::RangeBounds;

/// Spacer widget
#[derive(Default)]
pub struct Spacer {
    constraints: Constraints,
}

impl Spacer {
    /// Adjust horizontal spacing (constraints)
    ///
    /// ```rust
    /// use semtext::Spacer;
    ///
    /// let s0 = Spacer::default().with_horizontal(..10);
    /// let s1 = Spacer::default().with_horizontal(2..);
    /// ```
    pub fn with_horizontal<R>(mut self, horiz: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.constraints = self.constraints.with_horizontal(horiz);
        self
    }

    /// Adjust vertical spacing (constraints)
    ///
    /// ```rust
    /// use semtext::Spacer;
    ///
    /// let s0 = Spacer::default().with_vertical(1..8);
    /// let s1 = Spacer::default().with_vertical(2..=4);
    /// ```
    pub fn with_vertical<R>(mut self, vert: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.constraints = self.constraints.with_vertical(vert);
        self
    }
}

impl Widget for Spacer {
    /// Get the widget constraints
    fn constraints(&self) -> Constraints {
        self.constraints
    }
}
