// screen.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{BBox, Dim, Error, Result};
use crossterm::event::Event;
use crossterm::{cursor, event, queue, style, terminal};
use std::convert::TryFrom;
use std::io::{Stdout, Write};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Inner enum for glyphs
#[derive(Clone, Copy, Debug, PartialEq)]
enum GlyphInner<'a> {
    /// Character glyph
    Char(char),
    /// String slice glyph
    Str(&'a str),
}

/// Printable glyph
///
/// A glyph can be made from a `char` or `&str`:
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use semtext::Glyph;
/// use std::convert::TryFrom;
///
/// let glyph_char = Glyph::try_from('🦀')?;
/// let glyph_str = Glyph::try_from("a\u{308}")?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glyph<'a> {
    /// Inner glyph value (char or str)
    inner: GlyphInner<'a>,
    /// Width in text cells (must be either 1 or 2)
    width: usize,
}

/// Terminal screen
pub struct Screen {
    /// Standard Output
    out: Stdout,
    /// Dimensions of screen in text cells
    dim: Dim,
}

/// Cells of text
///
/// The cells are in a rectangular area of the screen.
pub struct Cells<'a> {
    /// Screen containing cells
    screen: &'a mut Screen,
    /// Bounding box of cells
    bbox: BBox,
}

impl TryFrom<char> for Glyph<'_> {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        if let Some(width) = c.width() {
            if width == 1 || width == 2 {
                let inner = GlyphInner::Char(c);
                return Ok(Glyph { inner, width });
            }
        }
        Err(Error::InvalidGlyph())
    }
}

impl<'a> TryFrom<&'a str> for Glyph<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let width = s.width();
        if width == 1 || width == 2 {
            let inner = GlyphInner::Str(s);
            return Ok(Glyph { inner, width });
        }
        Err(Error::InvalidGlyph())
    }
}

impl<'a> Glyph<'a> {
    /// Get the glyph width.
    ///
    /// The width must be either 1 or 2 (checked on construction).
    fn width(self) -> usize {
        self.width
    }
}

impl Screen {
    /// Create a new Screen
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        let dim = Dim::new(width, height);
        terminal::enable_raw_mode()?;
        let mut out = std::io::stdout();
        queue!(
            out,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::DisableLineWrap,
            terminal::Clear(terminal::ClearType::All),
        )?;
        Ok(Screen { out, dim })
    }

    /// Set the screen title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        queue!(self.out, terminal::SetTitle(title))?;
        Ok(())
    }

    /// Get the screen bounding box
    pub fn bbox(&self) -> BBox {
        BBox::new(0, 0, self.dim.width, self.dim.height)
    }

    /// Clear the screen (fill with the space character)
    pub fn clear(&mut self) -> Result<()> {
        queue!(self.out, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    /// Get all cells on screen
    pub fn cells(&mut self, bbox: BBox) -> Cells {
        let bbox = self.bbox().clip(bbox);
        Cells { screen: self, bbox }
    }

    /// Set a text attribute
    pub fn set_attribute(&mut self, attr: style::Attribute) -> Result<()> {
        queue!(self.out, style::SetAttribute(attr))?;
        Ok(())
    }

    /// Set the foreground color
    pub fn set_foreground_color(&mut self, color: style::Color) -> Result<()> {
        queue!(self.out, style::SetForegroundColor(color))?;
        Ok(())
    }

    /// Set the background color
    pub fn set_background_color(&mut self, color: style::Color) -> Result<()> {
        queue!(self.out, style::SetBackgroundColor(color))?;
        Ok(())
    }

    /// Move cursor to a cell
    fn move_to(&mut self, col: u16, row: u16) -> Result<()> {
        queue!(self.out, cursor::MoveTo(col, row))?;
        Ok(())
    }

    /// Move cursor right by a number of columns
    pub fn move_right(&mut self, col: u16) -> Result<()> {
        queue!(self.out, cursor::MoveRight(col))?;
        Ok(())
    }

    /// Print a char at the cursor location
    fn print_char(&mut self, ch: char) -> Result<()> {
        queue!(self.out, style::Print(ch))?;
        Ok(())
    }

    /// Print a str at the cursor location
    fn print_str(&mut self, st: &str) -> Result<()> {
        queue!(self.out, style::Print(st))?;
        Ok(())
    }

    /// Wait for input events
    pub fn event(&mut self) -> Result<Event> {
        self.out.flush()?;
        let ev = event::read()?;
        if let Event::Resize(width, height) = ev {
            self.dim = Dim::new(width, height);
        }
        return Ok(ev);
    }

    /// Cleanup screen
    fn cleanup(&mut self) -> Result<()> {
        queue!(
            self.out,
            terminal::EnableLineWrap,
            terminal::LeaveAlternateScreen,
            cursor::Show,
            style::ResetColor,
        )?;
        self.out.flush()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        if let Err(err) = self.cleanup() {
            // Is this useful?
            dbg!(err);
        }
    }
}

impl<'a> Cells<'a> {
    /// Get the width
    pub fn width(&self) -> u16 {
        self.bbox.width()
    }

    /// Get the height
    pub fn height(&self) -> u16 {
        self.bbox.height()
    }

    /// Fill the cells with a glyph
    pub fn fill(&'a mut self, glyph: Glyph<'a>) -> Result<()> {
        let bbox = self.bbox;
        let fill_width = bbox.width() / glyph.width() as u16;
        if bbox.height() > 0 && fill_width > 0 {
            self.move_to(bbox.col(), bbox.row())?;
            for row in 0..bbox.height() {
                self.move_to(bbox.col(), bbox.row() + row)?;
                for _ in 0..fill_width {
                    match glyph.inner {
                        GlyphInner::Char(ch) => self.screen.print_char(ch)?,
                        GlyphInner::Str(st) => self.screen.print_str(st)?,
                    }
                }
            }
        }
        Ok(())
    }

    /// Move cursor to a cell
    pub fn move_to(&mut self, col: u16, row: u16) -> Result<()> {
        let col = self.bbox.col() + col;
        let row = self.bbox.row() + row;
        self.screen.move_to(col, row)
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
