// traits.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, Event};
use crate::layout::{AreaBound, Cells};
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

    /// Handle event input
    fn event_input(&self, _event: Event) -> Option<Action> {
        // ignore by default
        None
    }

    /// Offer focus to widget
    fn focus_offer(&self) -> Option<Action> {
        None
    }

    /// Take focus from widget
    fn focus_take(&self) -> Option<Action> {
        None
    }

    /// Mouse hover over widget
    fn hover(&self) -> Option<Action> {
        None
    }
}
