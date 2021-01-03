// screen.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, Event, KeyMap, ModKeys, MouseEvent};
use crate::layout::{BBox, Cells, Dim, GridArea, Pos};
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

    /// Draw a grid area layout
    fn draw(&mut self, widget_boxes: &[(&dyn Widget, BBox)]) -> Result<()> {
        let background = self.theme.background;
        self.set_background_color(background)?;
        self.clear()?;
        for (widget, bbox) in widget_boxes.iter() {
            if let Some(border) = widget.border() {
                if let Some(mut cells) = self.cells(*bbox) {
                    border.draw(&mut cells)?;
                }
                if let Some(mut cells) = self.cells(border.inset(*bbox)) {
                    widget.draw(&mut cells)?;
                }
            } else if let Some(mut cells) = self.cells(*bbox) {
                widget.draw(&mut cells)?;
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
    ) -> Option<Action> {
        match ev {
            Event::Resize(dim) => {
                self.dim = dim;
                Some(Action::Resize(dim))
            }
            Event::Key(key, mods) => {
                // FIXME: check focused widget first
                self.keymap.lookup(key, mods)
            }
            Event::Mouse(mev, mods, pos) => {
                mouse_action(mev, mods, pos, widget_boxes)
            }
        }
    }

    /// Draw a grid area and wait for an action
    pub fn step(&mut self, area: &GridArea) -> Result<Action> {
        let widget_boxes = area.widget_boxes(self.bbox())?;
        self.draw(&widget_boxes)?;
        loop {
            let ev = Event::read()?;
            if let Some(action) = self.event_action(ev, &widget_boxes) {
                return Ok(action);
            }
        }
    }

    /// Render a grid area and wait asynchronously for an action
    #[cfg(feature = "async")]
    pub async fn step_future(&mut self, area: &GridArea<'_>) -> Result<Action> {
        let widget_boxes = area.widget_boxes(self.bbox())?;
        self.draw(&widget_boxes)?;
        loop {
            let ev = (&mut self.ev_stream).await.unwrap()?.into();
            if let Some(action) = self.event_action(ev, &widget_boxes) {
                return Ok(action);
            }
        }
    }

    /// Cleanup screen
    fn cleanup(&mut self) -> Result<()> {
        queue!(
            self.out,
            event::DisableMouseCapture,
            terminal::LeaveAlternateScreen,
            terminal::EnableLineWrap,
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

/// Handle a mouse action
fn mouse_action(
    mev: MouseEvent,
    mods: ModKeys,
    pos: Pos,
    widget_boxes: &[(&dyn Widget, BBox)],
) -> Option<Action> {
    let mut action = None;
    for (widget, bbox) in widget_boxes.iter() {
        let a = match (mev, bbox.within(pos)) {
            (MouseEvent::ButtonDown(_), Some(_)) => widget.focus_offer(),
            (MouseEvent::ButtonDown(_), None) => widget.focus_take(),
            (MouseEvent::Drag(None), Some(_)) => widget.hover_inside(),
            (MouseEvent::Drag(None), None) => widget.hover_outside(),
            _ => None,
        };
        action = action.or(a);
        // Only widget within bounds receives event
        if let Some(p) = bbox.within(pos) {
            let a = widget.event_input(Event::Mouse(mev, mods, p));
            action = action.or(a);
        }
    }
    action
}
