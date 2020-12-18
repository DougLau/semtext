// screen.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{BBox, Cells, Dim, GridArea};
use crate::style::{Color, Theme};
use crate::{Result, Widget};
use crossterm::event::Event;
use crossterm::{cursor, event, queue, style, terminal};
use std::io::{Stdout, Write};

#[cfg(feature = "async")]
use futures_core::stream::Stream;

#[cfg(feature = "async")]
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(feature = "async")]
/// Needed in order to await the stream.
struct EvStreamFut(Box<dyn Stream<Item = crossterm::Result<Event>> + Unpin>);

#[cfg(feature = "async")]
impl Future for EvStreamFut {
    type Output = Option<crossterm::Result<Event>>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

/// Terminal screen
pub struct Screen {
    /// Standard Output
    out: Stdout,
    /// Dimensions of screen in text cells
    dim: Dim,
    /// Style theme
    theme: Theme,
    #[cfg(feature = "async")]
    /// Event stream future.
    ev_stream: EvStreamFut,
}

impl Screen {
    /// Create a new Screen
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        let dim = Dim::new(width, height);
        let theme = Theme::default();
        terminal::enable_raw_mode()?;
        let mut out = std::io::stdout();
        queue!(
            out,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::DisableLineWrap,
            terminal::Clear(terminal::ClearType::All),
        )?;
        #[cfg(not(feature = "async"))]
        {
            Ok(Screen { out, dim, theme })
        }
        #[cfg(feature = "async")]
        {
            let ev_stream = EvStreamFut(Box::new(event::EventStream::new()));
            Ok(Screen {
                out,
                dim,
                theme,
                ev_stream,
            })
        }
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

    /// Get the theme
    pub(crate) fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Clear the screen (fill with the space character)
    fn clear(&mut self) -> Result<()> {
        queue!(self.out, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    /// Get cells contained by a bounding box
    fn cells(&mut self, bbox: BBox) -> Option<Cells> {
        let bbox = self.bbox().clip(bbox);
        if bbox.is_empty() {
            None
        } else {
            Some(Cells::new(self, bbox))
        }
    }

    /// Set a text attribute
    pub(crate) fn set_attribute(
        &mut self,
        attr: style::Attribute,
    ) -> Result<()> {
        queue!(self.out, style::SetAttribute(attr))?;
        Ok(())
    }

    /// Set the foreground color
    pub(crate) fn set_foreground_color(&mut self, color: Color) -> Result<()> {
        queue!(self.out, style::SetForegroundColor(color.into()))?;
        Ok(())
    }

    /// Set the background color
    pub(crate) fn set_background_color(&mut self, color: Color) -> Result<()> {
        queue!(self.out, style::SetBackgroundColor(color.into()))?;
        Ok(())
    }

    /// Move cursor to a cell
    pub(crate) fn move_to(&mut self, col: u16, row: u16) -> Result<()> {
        queue!(self.out, cursor::MoveTo(col, row))?;
        Ok(())
    }

    /// Move cursor right by a number of columns
    pub(crate) fn move_right(&mut self, col: u16) -> Result<()> {
        queue!(self.out, cursor::MoveRight(col))?;
        Ok(())
    }

    /// Print a char at the cursor location
    pub(crate) fn print_char(&mut self, ch: char) -> Result<()> {
        queue!(self.out, style::Print(ch))?;
        Ok(())
    }

    /// Print a str at the cursor location
    pub(crate) fn print_str(&mut self, st: &str) -> Result<()> {
        queue!(self.out, style::Print(st))?;
        Ok(())
    }

    /// Render a grid area layout
    pub fn render<'a>(&mut self, layout: &GridArea<'a>) -> Result<()> {
        let background = self.theme.background();
        self.set_background_color(background)?;
        self.clear()?;
        for (widget, bbox) in layout.widgets.iter().zip(&layout.boxes) {
            if let Some(border) = widget.border() {
                if let Some(mut cells) = self.cells(*bbox) {
                    border.render(&mut cells)?;
                }
                if let Some(mut cells) = self.cells(border.inset(*bbox)) {
                    widget.render(&mut cells)?;
                }
            } else if let Some(mut cells) = self.cells(*bbox) {
                widget.render(&mut cells)?;
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
        Ok(ev)
    }

    /// Asynchronous wait for input events
    #[cfg(feature = "async")]
    pub async fn input(&mut self) -> Result<Event> {
        self.out.flush()?;
        let ev = (&mut self.ev_stream).await.unwrap()?;
        if let Event::Resize(width, height) = ev {
            self.dim = Dim::new(width, height);
        }
        Ok(ev)
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
