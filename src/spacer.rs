// spacer.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{AreaBound, Cells, Glyph, IntoGlyph, Result, Widget};
use std::ops::RangeBounds;

/// Spacer widget
///
/// A spacer can be used for fixed or variable width spacing between other
/// widgets.  By default, it does not render anything, but a fill glyph may be
/// provided.
#[derive(Default)]
pub struct Spacer {
    /// Area bounds
    bounds: AreaBound,
    /// Fill character
    fill: Option<Glyph>,
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

    /// Set a fill glyph
    pub fn with_fill<G: IntoGlyph>(mut self, fill: G) -> Result<Self> {
        self.fill = Some(fill.into_glyph()?);
        Ok(self)
    }
}

impl Widget for Spacer {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        self.bounds
    }

    /// Render the widget
    fn render(&self, cells: &mut Cells) -> Result<()> {
        if let Some(fill) = &self.fill {
            cells.fill(fill)?;
        }
        Ok(())
    }
}
