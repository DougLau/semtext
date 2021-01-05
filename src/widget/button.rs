// button.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{AreaBound, Cells, Pos};
use crate::text::{IntoGlyph, StyleGroup, Theme};
use crate::widget::{Border, Label};
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
    /// Contained label widget
    lbl: Label,
    /// Button state
    state: Cell<State>,
}

impl Button {
    /// Create a new button widget
    pub fn new(txt: &str) -> Self {
        let lbl = Label::new(txt);
        let state = Cell::new(State::Enabled);
        Self { lbl, state }
    }

    /// Add a border around a button
    pub fn with_border(self) -> Border<Self> {
        Border::new(self)
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
}

impl Widget for Button {
    /// Get the style group
    fn style_group(&self) -> StyleGroup {
        match self.state.get() {
            State::Disabled => StyleGroup::Disabled,
            State::Enabled => StyleGroup::Enabled,
            State::Focused => StyleGroup::Focused,
            State::Hovered => StyleGroup::Hovered,
            State::Pressed => StyleGroup::Interacted,
        }
    }

    /// Get the area bounds
    fn bounds(&self, theme: &Theme) -> AreaBound {
        self.lbl.bounds(theme)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let theme = cells.theme();
        let style = theme.style(self.style_group());
        cells.set_style(style)?;
        // FIXME: maybe add a print_text variant that fills...
        cells.fill(&' '.into_glyph()?)?;
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
