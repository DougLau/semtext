// spacer.rs
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use crate::layout::{Cells, LengthBound, Pos};
use crate::text::{Glyph, IntoGlyph, Theme};
use crate::{Result, Widget};
use std::ops::RangeBounds;

/// Spacer widget
///
/// A spacer can be used for fixed or variable width spacing between other
/// widgets.  By default it draws nothing, but a fill glyph may be provided
/// using [with_fill].
///
/// [with_fill]: struct.Spacer.html#method.with_fill
#[derive(Default)]
pub struct Spacer {
    /// Width bounds
    width_bounds: LengthBound,
    /// Height bounds
    height_bounds: LengthBound,
    /// Fill character
    fill: Option<Glyph>,
}

impl Spacer {
    /// Adjust column spacing (bounds)
    ///
    /// ```rust
    /// use semtext::widget::Spacer;
    ///
    /// let s0 = Spacer::default().with_columns(..10);
    /// let s1 = Spacer::default().with_columns(2..);
    /// ```
    pub fn with_columns<R>(mut self, col: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.width_bounds = LengthBound::new(col);
        self
    }

    /// Adjust row spacing (bounds)
    ///
    /// ```rust
    /// use semtext::widget::Spacer;
    ///
    /// let s0 = Spacer::default().with_rows(1..8);
    /// let s1 = Spacer::default().with_rows(2..=4);
    /// ```
    pub fn with_rows<R>(mut self, row: R) -> Self
    where
        R: RangeBounds<u16>,
    {
        self.height_bounds = LengthBound::new(row);
        self
    }

    /// Set a fill glyph
    pub fn with_fill<G: IntoGlyph>(mut self, fill: G) -> Result<Self> {
        self.fill = Some(fill.into_glyph()?);
        Ok(self)
    }
}

impl Widget for Spacer {
    /// Get the width bounds
    fn width_bounds(&self, _theme: &Theme) -> LengthBound {
        self.width_bounds
    }

    /// Get the height bounds
    fn height_bounds(&self, _theme: &Theme, _width: u16) -> LengthBound {
        self.height_bounds
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, _offset: Pos) -> Result<()> {
        if let Some(fill) = &self.fill {
            cells.fill(fill)?;
        }
        Ok(())
    }
}
