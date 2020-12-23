// button.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, Event, MouseEvent};
use crate::layout::{AreaBound, Cells};
use crate::text::{Outline, Style, Theme};
use crate::widget::{Border, Label};
use crate::{Result, Widget};
use std::cell::Cell;

/// Button state
#[derive(Clone, Copy)]
enum State {
    /// Enabled (with focus)
    Enabled,
    /// Clicked (with focus)
    Clicked,
}

/// Button widget
pub struct Button {
    lbl: Label,
    state: Cell<State>,
}

impl Button {
    /// Create a new button widget
    pub fn new(txt: &str) -> Self {
        let lbl = Label::new(txt);
        let state = Cell::new(State::Enabled);
        Self { lbl, state }
    }

    /// Get button style based on current state
    pub fn style(&self, theme: &Theme) -> Style {
        match self.state.get() {
            State::Enabled => Style::default()
                .with_background(theme.background())
                .with_foreground(theme.foreground()),
            State::Clicked => Style::default()
                .with_background(theme.tertiary())
                .with_foreground(theme.background()),
        }
    }
}

impl Widget for Button {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        self.lbl.bounds()
    }

    /// Get the border
    fn border(&self) -> Option<Border> {
        Some(match self.state.get() {
            State::Enabled => Border::default()
                .with_bottom(Some(Outline::HalfInner))
                .with_right(Some(Outline::HalfInner)),
            State::Clicked => Border::default()
                .with_top(Some(Outline::HalfInner))
                .with_right(Some(Outline::HalfInner)),
        })
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let theme = cells.theme();
        let style = self.style(theme);
        cells.set_style(style)?;
        cells.print_text(self.lbl.txt())
    }

    /// Handle event input
    fn event_input(&self, event: Event) -> Option<Action> {
        use MouseEvent::*;
        match event {
            Event::Mouse(ButtonDown(_), _, _) => Some(State::Clicked),
            Event::Mouse(ButtonUp(_), _, _) => Some(State::Enabled),
            _ => None,
        }
        .and_then(|s| {
            self.state.set(s);
            Some(Action::Redraw())
        })
    }
}
