// cells.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{BBox, Glyph};
use crate::text::{Color, Theme};
use crate::{Result, Screen};

/// Cells of text on a [Screen]
///
/// The cells are in a rectangular area of the screen.
pub struct Cells<'a> {
    /// Screen containing cells
    screen: &'a mut Screen,
    /// Bounding box of cells
    bbox: BBox,
}

impl<'a> Cells<'a> {
    /// Create cells
    pub fn new(screen: &'a mut Screen, bbox: BBox) -> Self {
        Self { screen, bbox }
    }

    /// Get the width
    pub fn width(&self) -> u16 {
        self.bbox.width()
    }

    /// Get the height
    pub fn height(&self) -> u16 {
        self.bbox.height()
    }

    /// Fill the cells with a glyph
    pub fn fill(&mut self, glyph: &Glyph) -> Result<()> {
        let bbox = self.bbox;
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

    /// Set the foreground color
    pub fn set_foreground_color(&mut self, color: Color) -> Result<()> {
        self.screen.set_foreground_color(color)
    }

    /// Set the background color
    pub fn set_background_color(&mut self, color: Color) -> Result<()> {
        self.screen.set_background_color(color)
    }

    /// Move cursor to a cell
    pub fn move_to(&mut self, col: u16, row: u16) -> Result<()> {
        let col = self.bbox.left() + col;
        let row = self.bbox.top() + row;
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
}
