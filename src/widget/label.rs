// label.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, Cells};
use crate::{Result, Widget};
use unicode_width::UnicodeWidthStr;

/// Text label widget
pub struct Label {
    txt: String,
}

impl Label {
    /// Create a new label widget
    pub fn new(txt: &str) -> Self {
        let txt = txt.to_string();
        Label { txt }
    }

    /// Get text
    pub fn txt(&self) -> &str {
        &self.txt
    }
}

impl Widget for Label {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        let b = AreaBound::default();
        let w = self.txt.width() as u16;
        let rows = w / 24 + 1;
        let cols = w / rows + 1;
        b.with_columns(cols..=cols + 2).with_rows(rows..=rows)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let style = cells.theme().style();
        cells.set_style(style)?;
        cells.print_text(&self.txt)
    }
}
