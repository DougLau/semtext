// screen.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Area, Dim, Error, Result};
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glyph<'a> {
    /// Inner glyph value (char or str)
    inner: GlyphInner<'a>,
    /// Width in grid cells (must be either 1 or 2)
    width: usize,
}

/// Terminal screen
pub struct Screen {
    /// Standard Output
    out: Stdout,
    /// Dimensions of screen in character cells
    dim: Dim,
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

    /// Get the total screen Area
    pub fn area(&self) -> Area {
        Area::new(0, 0, self.dim.width, self.dim.height)
    }

    /// Clear the screen (fill with the space character)
    pub fn clear(&mut self) -> Result<()> {
        queue!(self.out, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
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

    /// Move cursor to a grid cell
    pub fn move_to(&mut self, col: u16, row: u16) -> Result<()> {
        queue!(self.out, cursor::MoveTo(col, row))?;
        Ok(())
    }

    /// Move cursor right by a number of columns
    pub fn move_right(&mut self, col: u16) -> Result<()> {
        queue!(self.out, cursor::MoveRight(col))?;
        Ok(())
    }

    /// Print a glyph at the cursor location
    pub fn print<'a>(&mut self, glyph: Glyph<'a>) -> Result<()> {
        match glyph.inner {
            GlyphInner::Char(ch) => queue!(self.out, style::Print(ch))?,
            GlyphInner::Str(st) => queue!(self.out, style::Print(st))?,
        }
        Ok(())
    }

    /// Fill an area with a character
    pub fn fill<'a>(&mut self, area: Area, glyph: Glyph<'a>) -> Result<()> {
        let fill_width = area.width() / glyph.width() as u16;
        self.move_to(area.col(), area.row())?;
        for row in 0..area.height() {
            self.move_to(area.col(), area.row() + row)?;
            for _ in 0..fill_width {
                self.print(glyph)?;
            }
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
