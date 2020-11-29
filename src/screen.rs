// screen.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Area, Dim, Result};
use crossterm::event::Event;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen, SetTitle,
};
use crossterm::{cursor, event, queue, style};
use std::io::{Stdout, Write};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Inner enum for glyphs
#[derive(Clone, Copy, Debug)]
enum GlyphInner<'a> {
    /// Character glyph
    Char(char),
    /// String slice glyph
    Str(&'a str),
}

/// Printable glyph
#[derive(Clone, Copy, Debug)]
pub struct Glyph<'a> {
    inner: GlyphInner<'a>,
}

/// Terminal screen
pub struct Screen {
    /// Standard Output
    out: Stdout,
    /// Dimensions of screen in character cells
    dim: Dim,
}

impl From<char> for Glyph<'_> {
    fn from(c: char) -> Self {
        let inner = GlyphInner::Char(c);
        Glyph { inner }
    }
}

impl<'a> From<&'a str> for Glyph<'a> {
    fn from(s: &'a str) -> Self {
        let inner = GlyphInner::Str(s);
        Glyph { inner }
    }
}

impl<'a> Glyph<'a> {
    /// Get the glyph width
    fn width(self) -> usize {
        match self.inner {
            GlyphInner::Char(c) => {
                c.width().unwrap_or(0)
            }
            GlyphInner::Str(s) => {
                s.width()
            }
        }
    }
}

impl Screen {
    /// Create a new Screen
    pub fn new() -> Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        let dim = Dim::new(width, height);
        enable_raw_mode()?;
        let mut out = std::io::stdout();
        queue!(out, EnterAlternateScreen, cursor::Hide)?;
        Ok(Screen {
            out,
            dim,
        })
    }

    /// Set the screen title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        queue!(self.out, SetTitle(title))?;
        Ok(())
    }

    /// Get the total screen Area
    pub fn area(&self) -> Area {
        Area::new(0, 0, self.dim.width, self.dim.height)
    }

    /// Clear the screen to the background color
    pub fn clear(&mut self) -> Result<()> {
        queue!(self.out, Clear(ClearType::All))?;
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
    pub fn print<'a, G>(&mut self, g: G) -> Result<()>
        where G: Into<Glyph<'a>>,
    {
        let glyph = g.into();
        let width = glyph.width();
        debug_assert!(width == 1 || width == 2);
        match glyph.inner {
            GlyphInner::Char(c) => queue!(self.out, style::Print(c))?,
            GlyphInner::Str(s) => queue!(self.out, style::Print(s))?,
        }
        Ok(())
    }

    /// Fill an area with a character
    pub fn fill(&mut self, area: Area, fill: char) -> Result<()> {
        let glyph = Glyph::from(fill);
        let width = glyph.width();
        if width > 0 && width <= 2 {
            let fill_width = area.width() / width as u16;
            self.move_to(area.col(), area.row())?;
            for row in 0..area.height() {
                self.move_to(area.col(), area.row() + row)?;
                for _ in 0..fill_width {
                    self.print(glyph)?;
                }
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
            LeaveAlternateScreen,
            cursor::Show,
            style::ResetColor,
        )?;
        self.out.flush()?;
        disable_raw_mode()?;
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
