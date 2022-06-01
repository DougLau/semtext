// label.rs
//
// Copyright (c) 2020-2022  Douglas P Lau
//
use crate::layout::{Cells, LengthBound, Pos};
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
}

impl Widget for Label {
    /// Get the width bounds
    fn width_bounds(&self, _theme: &Theme) -> LengthBound {
        let w = self.text.width() as u16;
        match w {
            0..=8 => LengthBound::new(w..),
            9..=20 => LengthBound::new(10..),
            _ => LengthBound::new(12..),
        }
    }

    /// Get the height bounds
    fn height_bounds(&self, _theme: &Theme, width: u16) -> LengthBound {
        let rows = wrap(&self.text, usize::from(width)).len() as u16;
        LengthBound::new(rows..=rows)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, offset: Pos) -> Result<()> {
        cells.print_text(&self.text, offset)
    }
}
