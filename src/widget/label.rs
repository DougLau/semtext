// label.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, Cells};
use crate::text::{StyleGroup, Theme};
use crate::widget::Border;
use crate::{Result, Widget};
use unicode_width::UnicodeWidthStr;

/// Text label widget
pub struct Label {
    /// Text of label
    txt: String,
}

impl Label {
    /// Create a new label widget
    pub fn new(txt: &str) -> Self {
        let txt = txt.to_string();
        Label { txt }
    }

    /// Add a border around a label
    pub fn with_border(self) -> Border<Self> {
        Border::new(self)
    }

    /// Get label text
    pub fn txt(&self) -> &str {
        &self.txt
    }
}

impl Widget for Label {
    /// Get the area bounds
    fn bounds(&self, _theme: &Theme) -> AreaBound {
        let b = AreaBound::default();
        let w = self.txt.width() as u16;
        let rows = w / 24 + 1;
        let cols = w / rows + 1;
        b.with_columns(cols..=cols + 2).with_rows(rows..=rows)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let style = cells.theme().style(StyleGroup::Enabled);
        cells.set_style(style)?;
        cells.print_text(&self.txt)
    }
}
