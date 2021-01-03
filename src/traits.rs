// traits.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, ModKeys, MouseEvent};
use crate::layout::{AreaBound, Cells, Pos};
use crate::widget::Border;
use crate::Result;

/// A component of a user interface
pub trait Widget {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        AreaBound::default()
    }

    /// Get the border
    fn border(&self) -> Option<Border> {
        None
    }

    /// Draw the widget
    fn draw(&self, _cells: &mut Cells) -> Result<()> {
        // default implementation draws nothing
        Ok(())
    }

    /// Offer focus to widget
    fn focus_offer(&self) -> Option<Action> {
        None
    }

    /// Take focus from widget
    fn focus_take(&self) -> Option<Action> {
        None
    }

    /// Mouse hover inside widget bounds
    fn hover_inside(&self) -> Option<Action> {
        None
    }

    /// Mouse hover outside widget bounds
    fn hover_outside(&self) -> Option<Action> {
        None
    }

    /// Handle mouse events
    fn mouse_event(
        &self,
        _mev: MouseEvent,
        _mods: ModKeys,
        _pos: Pos,
    ) -> Option<Action> {
        // ignore by default
        None
    }
}
