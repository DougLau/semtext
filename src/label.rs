// label.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Cells, Result, Widget};
use textwrap::wrap_iter;

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
}

impl Widget for Label {
    /// Render the widget
    fn render(&self, cells: &mut Cells) -> Result<()> {
        let width = usize::from(cells.width());
        let height = usize::from(cells.height());
        for (row, txt) in wrap_iter(&self.txt, width).take(height).enumerate() {
            let row = row as u16; // limited to u16 by take(height)
            cells.move_to(0, row)?;
            cells.print_str(&txt)?;
        }
        Ok(())
    }
}
