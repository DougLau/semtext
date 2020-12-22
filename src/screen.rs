// screen.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, Event, KeyMap};
use crate::layout::{BBox, Cells, Dim, GridArea};
use crate::text::{Appearance, Color, Style, Theme};
use crate::{Result, Widget};
use crossterm::event::Event as CtEvent;
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
struct EvStreamFut(Box<dyn Stream<Item = crossterm::Result<CtEvent>> + Unpin>);

#[cfg(feature = "async")]
impl Future for EvStreamFut {
    type Output = Option<crossterm::Result<CtEvent>>;

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
    /// Current text style
    style: Option<Style>,
    /// Key / action map
    keymap: KeyMap,
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
        let style = None;
        let keymap = KeyMap::default();
        terminal::enable_raw_mode()?;
        let mut out = std::io::stdout();
        queue!(
            out,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::DisableLineWrap,
            terminal::Clear(terminal::ClearType::All),
            event::EnableMouseCapture,
        )?;
        #[cfg(not(feature = "async"))]
        {
            Ok(Screen {
                out,
                dim,
                theme,
                style,
                keymap,
            })
        }
        #[cfg(feature = "async")]
        {
            let ev_stream = EvStreamFut(Box::new(event::EventStream::new()));
            Ok(Screen {
                out,
                dim,
                theme,
                style,
                keymap,
                ev_stream,
            })
        }
    }

    /// Set the key / action map
    pub fn set_keymap(&mut self, keymap: KeyMap) {
        self.keymap = keymap;
    }

    /// Set the screen title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        queue!(self.out, terminal::SetTitle(title))?;
        Ok(())
    }

    /// Get the screen bounding box
    fn bbox(&self) -> BBox {
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

    /// Set the background color
    fn set_background_color(&mut self, color: Color) -> Result<()> {
        if self.style.map_or(true, |s| s.background() != color) {
            queue!(self.out, style::SetBackgroundColor(color.into()))?;
        }
        Ok(())
    }

    /// Set the foreground color
    fn set_foreground_color(&mut self, color: Color) -> Result<()> {
        if self.style.map_or(true, |s| s.foreground() != color) {
            queue!(self.out, style::SetForegroundColor(color.into()))?;
        }
        Ok(())
    }

    /// Set the text appearance
    fn set_appearance(&mut self, app: Appearance) -> Result<()> {
        let attrs = app.changed(
            self.style.map_or(Appearance::default(), |s| s.appearance()),
        );
        if !attrs.is_empty() {
            queue!(self.out, style::SetAttributes(attrs))?;
        }
        Ok(())
    }

    /// Set the text style
    pub(crate) fn set_style(&mut self, st: Style) -> Result<()> {
        self.set_background_color(st.background())?;
        self.set_foreground_color(st.foreground())?;
        self.set_appearance(st.appearance())?;
        self.style = Some(st);
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
    fn render(&mut self, widget_boxes: &[(&dyn Widget, BBox)]) -> Result<()> {
        let background = self.theme.background();
        self.set_background_color(background)?;
        self.clear()?;
        for (widget, bbox) in widget_boxes.iter() {
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
        self.out.flush()?;
        Ok(())
    }

    /// Check an event for an action
    fn event_action(
        &mut self,
        ev: Event,
        widget_boxes: &[(&dyn Widget, BBox)],
    ) -> Result<Option<Action>> {
        // FIXME: check widgets first
        match ev {
            Event::Resize(dim) => {
                self.dim = dim;
                return Ok(Some(Action::Resize()));
            }
            Event::Key(key, mods) => {
                if let Some(action) = self.keymap.lookup(key, mods) {
                    return Ok(Some(action));
                }
            }
            Event::Mouse(_mev, _mods, pos) => {
                for (_widget, bbox) in widget_boxes.iter() {
                    if let Some(_p) = bbox.within(pos) {
                        break;
                    }
                }
            }
        }
        Ok(None)
    }

    /// Render a grid area and wait for an action
    pub fn step(&mut self, area: &GridArea) -> Result<Action> {
        let widget_boxes = area.widget_boxes(self.bbox())?;
        self.render(&widget_boxes)?;
        loop {
            let ev = Event::read()?;
            if let Some(action) = self.event_action(ev, &widget_boxes)? {
                return Ok(action);
            }
        }
    }

    /// Render a grid area and wait asynchronously for an action
    #[cfg(feature = "async")]
    pub async fn step_future(&mut self, area: &GridArea<'_>) -> Result<Action> {
        let widget_boxes = area.widget_boxes(self.bbox())?;
        self.render(&widget_boxes)?;
        loop {
            let ev = (&mut self.ev_stream).await.unwrap()?.into();
            if let Some(action) = self.event_action(ev, &widget_boxes)? {
                return Ok(action);
            }
        }
    }

    /// Cleanup screen
    fn cleanup(&mut self) -> Result<()> {
        queue!(
            self.out,
            event::DisableMouseCapture,
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
