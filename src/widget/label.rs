// label.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::AreaBound;
use crate::style::Outline;
use crate::widget::Border;
use crate::{Cells, Result, Widget};
use textwrap::wrap_iter;
use unicode_width::UnicodeWidthStr;

/// Text label widget
///
/// Inline styling using Markdown:
///
/// Text Style        | Markdown
/// ------------------|---------
/// Normal            | `Normal`
/// `Reverse`         | `` `Reverse` ``
/// _Italic_          | `*Italic*` or `_Italic_`
/// **Bold**          | `**Bold**` or `__Bold__`
/// ~~Strikethrough~~ | `~~Strikethrough~~`
/// <u>Underline</u>  | `<u>Underline</u>`
///
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
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        let b = AreaBound::default();
        let w = self.txt.width() as u16;
        let rows = w / 24 + 1;
        let cols = w / rows + 1;
        b.with_columns(cols..=cols + 2).with_rows(rows..=rows)
    }

    /// Get the border
    fn border(&self) -> Option<Border> {
        Some(Border::default().with_all(Outline::Solid))
    }

    /// Render the widget
    fn render(&self, cells: &mut Cells) -> Result<()> {
        let foreground = cells.theme().foreground();
        let background = cells.theme().background();
        cells.set_foreground_color(foreground)?;
        cells.set_background_color(background)?;
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
