// screen.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{BBox, Dim, Error, Layout, Result};
use crossterm::event::Event;
use crossterm::{cursor, event, queue, style, terminal};
use std::io::{Stdout, Write};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Inner enum for glyphs
#[derive(Clone, Debug, PartialEq)]
enum GlyphInner {
    /// Character glyph
    Char(char),
    /// String glyph
    Str(String),
}

/// Printable glyph
///
/// A glyph can be made from a `char` or `&str`:
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use semtext::IntoGlyph;
///
/// let glyph_char = '🦀'.into_glyph()?;
/// let glyph_str = "a\u{308}".into_glyph()?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Glyph {
    /// Inner glyph value (char or String)
    inner: GlyphInner,
    /// Width in text cells (must be either 1 or 2)
    width: usize,
}

/// Trait to convert into a Glyph
///
/// This is used instead of TryFrom to avoid error conversion nonsense
pub trait IntoGlyph {
    fn into_glyph(self) -> Result<Glyph>;
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

impl IntoGlyph for char {
    /// Create a Glyph from a `char`
    fn into_glyph(self) -> Result<Glyph> {
        if let Some(width) = self.width() {
            if width == 1 || width == 2 {
                let inner = GlyphInner::Char(self);
                return Ok(Glyph { inner, width });
            }
        }
        Err(Error::InvalidGlyph())
    }
}

impl IntoGlyph for &str {
    /// Create a Glyphn from a `&str`
    fn into_glyph(self) -> Result<Glyph> {
        let width = self.width();
        if width == 1 || width == 2 {
            let inner = GlyphInner::Str(self.to_string());
            return Ok(Glyph { inner, width });
        }
        Err(Error::InvalidGlyph())
    }
}

impl Glyph {
    /// Get the glyph width.
    ///
    /// The width must be either 1 or 2 (checked on construction).
    fn width(&self) -> usize {
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

    /// Render a layout
    pub fn render<'a>(&mut self, layout: &Layout<'a>) -> Result<()> {
        for (widget, bbox) in layout.widgets.iter().zip(&layout.boxes) {
            let mut cells = self.cells(*bbox);
            widget.render(&mut cells)?;
        }
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
    pub fn fill(&mut self, glyph: &Glyph) -> Result<()> {
        let bbox = self.bbox;
        let fill_width = bbox.width() / glyph.width() as u16;
        if bbox.height() > 0 && fill_width > 0 {
            self.move_to(bbox.left(), bbox.top())?;
            for row in 0..bbox.height() {
                self.move_to(bbox.left(), bbox.top() + row)?;
                for _ in 0..fill_width {
                    match &glyph.inner {
                        GlyphInner::Char(ch) => self.screen.print_char(*ch)?,
                        GlyphInner::Str(st) => self.screen.print_str(&st)?,
                    }
                }
            }
        }
        Ok(())
    }

    /// Move cursor to a cell
    pub fn move_to(&mut self, col: u16, row: u16) -> Result<()> {
        let col = self.bbox.left() + col;
        let row = self.bbox.top() + row;
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
