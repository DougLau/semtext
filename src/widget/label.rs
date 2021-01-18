// label.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, Cells, Pos};
use crate::text::Theme;
use crate::{Result, Widget};
use textwrap::wrap;
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

    /// Calculate the label column width
    fn columns(&self) -> u16 {
        match self.text.width() {
            0..=2 => 2,
            3..=4 => 4,
            5..=6 => 6,
            6..=12 => 8,
            12..=20 => 10,
            20..=32 => 12,
            32..=48 => 14,
            _ => 16,
        }
    }
}

impl Widget for Label {
    /// Get the area bounds
    fn bounds(&self, _theme: &Theme) -> AreaBound {
        let cols = self.columns();
        let rows = wrap(&self.text, usize::from(cols)).iter().count() as u16;
        AreaBound::default()
            .with_columns(cols..=cols)
            .with_rows(rows..=rows)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, offset: Pos) -> Result<()> {
        cells.print_text(&self.text, offset)
    }
}
