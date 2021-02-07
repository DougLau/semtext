// button.rs
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{Cells, Dim, LengthBound, Pos};
use crate::text::{IntoGlyph, StyleGroup, Theme, WidgetGroup};
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
pub struct Button<W: Widget> {
    /// Wrapped widget
    wrapped: W,
    /// Button state
    state: Cell<State>,
}

impl<W: Widget> Button<W> {
    /// Create a new button widget
    pub fn new(wrapped: W) -> Self {
        let state = Cell::new(State::Enabled);
        Self { wrapped, state }
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

impl<W: Widget> Widget for Button<W> {
    /// Get the widget group
    fn widget_group(&self) -> WidgetGroup {
        WidgetGroup::Button
    }

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

    /// Get the width bounds
    fn width_bounds(&self, theme: &Theme) -> LengthBound {
        self.wrapped.width_bounds(theme)
    }

    /// Get the height bounds
    fn height_bounds(&self, theme: &Theme, width: u16) -> LengthBound {
        self.wrapped.height_bounds(theme, width)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, offset: Pos) -> Result<()> {
        // FIXME: maybe add a print_text variant that fills...
        cells.fill(&' '.into_glyph()?)?;
        self.wrapped.draw(cells, offset)
    }

    /// Handle focus event
    fn focus(&self, fev: FocusEvent) -> Option<Action> {
        use FocusEvent::*;
        use State::*;
        let state = self.state.get();
        match (fev, state) {
            (_, Disabled) => Some(Disabled),
            (Offer, _) => Some(Focused),
            (Take, _) => Some(Enabled),
            (HoverInside, Enabled) => Some(Hovered),
            (HoverInside, Pressed) => Some(Focused),
            (HoverOutside, Hovered) => Some(Enabled),
            (HoverOutside, Pressed) => Some(Focused),
            _ => None,
        }
        .and_then(|st| {
            if st != self.state.get() {
                self.state.set(st);
                Some(Action::Redraw())
            } else {
                None
            }
        })
    }

    /// Handle mouse events
    fn mouse_event(
        &self,
        mev: MouseEvent,
        _mods: ModKeys,
        _dim: Dim,
        _pos: Pos,
    ) -> Option<Action> {
        let state = self.state.get();
        match (mev, state) {
            (_, State::Disabled) => None,
            (MouseEvent::ButtonDown(_), _) => Some(State::Pressed),
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
