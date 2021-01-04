// button.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{AreaBound, Cells, Pos};
use crate::text::{Outline, Style, Theme};
use crate::widget::{Border, BorderHeight, BorderStyle, Label};
use crate::{Result, Widget};
use std::cell::Cell;

/// Button state
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    /// Button disabled
    Disabled,
    /// Button enabled
    Enabled,
    /// Button hovered
    Hovered,
    /// Button focused
    Focused,
    /// Button pressed
    Pressed,
}

/// Button widget
pub struct Button {
    lbl: Label,
    state: Cell<State>,
    border_style: Cell<BorderStyle>,
}

impl Button {
    /// Create a new button widget
    pub fn new(txt: &str) -> Self {
        let lbl = Label::new(txt);
        let state = Cell::new(State::Enabled);
        let border_style = Cell::new(BorderStyle::Bevel(
            Outline::default(),
            BorderHeight::Raised,
        ));
        Self {
            lbl,
            state,
            border_style,
        }
    }

    /// Disable the button
    pub fn disable(&self) {
        self.state.set(State::Disabled);
    }

    /// Enable the button
    pub fn enable(&self) {
        if self.state.get() == State::Disabled {
            self.state.set(State::Enabled);
        }
    }

    /// Get button style based on current state
    pub fn style(&self, theme: &Theme) -> Style {
        self.border_style.set(theme.button_border);
        match self.state.get() {
            State::Disabled => Style::default()
                .with_background(theme.background)
                .with_foreground(theme.dark_shadow),
            State::Enabled => Style::default()
                .with_background(theme.background)
                .with_foreground(theme.foreground),
            State::Focused => Style::default()
                .with_background(theme.secondary)
                .with_foreground(theme.background),
            State::Hovered => Style::default()
                .with_background(theme.background)
                .with_foreground(theme.secondary),
            State::Pressed => Style::default()
                .with_background(theme.tertiary)
                .with_foreground(theme.background),
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
        let state = self.state.get();
        let border_style = self.border_style.get();
        Some(match (state, border_style) {
            (State::Disabled, _) => Border::new(BorderStyle::Empty),
            (State::Pressed, BorderStyle::Bevel(outline, _)) => {
                Border::new(BorderStyle::Bevel(outline, BorderHeight::Lowered))
            }
            (_, BorderStyle::Bevel(outline, _)) => {
                Border::new(BorderStyle::Bevel(outline, BorderHeight::Raised))
            }
            (_, border_style) => Border::new(border_style),
        })
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let theme = cells.theme();
        let style = self.style(theme);
        cells.set_style(style)?;
        cells.print_text(self.lbl.txt())
    }

    /// Handle focus event
    fn focus(&self, fev: FocusEvent) -> Option<Action> {
        let state = self.state.get();
        use State::*;
        match (fev, state) {
            (FocusEvent::Offer, Enabled) => {
                self.state.set(Focused);
                Some(Action::Redraw())
            }
            (FocusEvent::Take, Focused)
            | (FocusEvent::Take, Hovered)
            | (FocusEvent::Take, Pressed) => {
                self.state.set(Enabled);
                Some(Action::Redraw())
            }
            (FocusEvent::HoverInside, Enabled) => {
                self.state.set(Hovered);
                Some(Action::Redraw())
            }
            (FocusEvent::HoverOutside, Pressed) => {
                self.state.set(State::Focused);
                Some(Action::Redraw())
            }
            (FocusEvent::HoverOutside, Hovered) => {
                self.state.set(State::Enabled);
                Some(Action::Redraw())
            }
            _ => None,
        }
    }

    /// Handle mouse events
    fn mouse_event(
        &self,
        mev: MouseEvent,
        _mods: ModKeys,
        _pos: Pos,
    ) -> Option<Action> {
        let state = self.state.get();
        match (mev, state) {
            (_, State::Disabled) => None,
            (MouseEvent::ButtonDown(_), _) => Some(State::Pressed),
            (MouseEvent::ButtonUp(_), State::Pressed) => Some(State::Focused),
            _ => None,
        }
        .and_then(|s| {
            if s != state {
                self.state.set(s);
                Some(Action::Redraw())
            } else {
                None
            }
        })
    }
}
