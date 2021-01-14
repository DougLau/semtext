// cells.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::BBox;
use crate::text::{Glyph, TextStyle, Theme};
use crate::{Result, Screen};
use textwrap::wrap_iter;

/// Cells of text on a [Screen]
///
/// The cells are in a rectangular area of the screen.
pub struct Cells<'a> {
    /// Screen containing cells
    screen: &'a mut Screen,
    /// Bounding box of cells
    bbox: BBox,
    /// Bounding box of clip area
    clip: BBox,
}

impl<'a> Cells<'a> {
    /// Create cells
    pub fn new(screen: &'a mut Screen, bbox: BBox) -> Self {
        let clip = bbox;
        Self { screen, bbox, clip }
    }

    /// Get the width
    pub fn width(&self) -> u16 {
        self.clip.width()
    }

    /// Get the height
    pub fn height(&self) -> u16 {
        self.clip.height()
    }

    /// Clip to bounding box
    pub fn clip(&mut self, inset: Option<BBox>) {
        if let Some(inset) = inset {
            let col = self.bbox.left() + inset.left();
            let row = self.bbox.top() + inset.top();
            let width = inset.width();
            let height = inset.height();
            self.clip = self.bbox.clip(BBox::new(col, row, width, height));
        } else {
            self.clip = self.bbox;
        }
    }

    /// Fill the cells with a glyph
    pub fn fill(&mut self, glyph: &Glyph) -> Result<()> {
        let bbox = self.clip;
        let fill_width = bbox.width() / glyph.width() as u16;
        if bbox.height() > 0 && fill_width > 0 {
            self.move_to(0, 0)?;
            for row in 0..bbox.height() {
                self.move_to(0, row)?;
                for _ in 0..fill_width {
                    glyph.print(&mut self.screen)?;
                }
            }
        }
        Ok(())
    }

    /// Get the screen theme
    pub fn theme(&self) -> &Theme {
        &self.screen.theme()
    }

    /// Set the text style
    pub fn set_style(&mut self, st: TextStyle) -> Result<()> {
        self.screen.set_style(st)
    }

    /// Move cursor to a cell
    pub fn move_to(&mut self, col: u16, row: u16) -> Result<()> {
        let col = self.clip.left() + col;
        let row = self.clip.top() + row;
        self.screen.move_to(col, row)
    }

    /// Move cursor right by a number of columns
    pub fn move_right(&mut self, col: u16) -> Result<()> {
        self.screen.move_right(col)
    }

    /// Print a char at the cursor location
    pub fn print_char(&mut self, ch: char) -> Result<()> {
        // FIXME: check width first
        self.screen.print_char(ch)
    }

    /// Print a str at the cursor location
    pub fn print_str(&mut self, st: &str) -> Result<()> {
        // FIXME: check width first
        self.screen.print_str(st)
    }

    /// Print some text
    ///
    /// Inline styling using Markdown:
    ///
    /// Text Style        | Markdown
    /// ------------------|---------
    /// Normal            | `Normal`
    /// _Italic_          | `*Italic*` or `_Italic_`
    /// **Bold**          | `**Bold**` or `__Bold__`
    /// ~~Strikethrough~~ | `~~Strikethrough~~`
    /// <u>Underline</u>  | `<u>Underline</u>`
    /// `Reverse`         | `` `Reverse` ``
    pub fn print_text(&mut self, text: &str) -> Result<()> {
        let width = usize::from(self.width());
        let height = usize::from(self.height());
        for (row, txt) in wrap_iter(&text, width).take(height).enumerate() {
            let row = row as u16; // limited to u16 by take(height)
            self.move_to(0, row)?;
            self.print_str(&txt)?;
        }
        Ok(())
    }
}
