// label.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, Cells, Pos};
use crate::text::Theme;
use crate::{Result, Widget};
use unicode_width::UnicodeWidthStr;

/// Text label widget
pub struct Label {
    /// Text of label
    text: String,
}

impl Label {
    /// Create a new label widget
    pub fn new(text: &str) -> Self {
        let text = text.to_string();
        Label { text }
    }

    /// Get label text
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Widget for Label {
    /// Get the area bounds
    fn bounds(&self, _theme: &Theme) -> AreaBound {
        let b = AreaBound::default();
        let w = self.text.width() as u16;
        let rows = w / 24 + 1;
        let cols = w / rows + 1;
        b.with_columns(cols..=cols + 2).with_rows(rows..=rows)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, pos: Pos) -> Result<()> {
        assert!(pos == Pos::default(), "FIXME");
        cells.print_text(&self.text)
    }
}
