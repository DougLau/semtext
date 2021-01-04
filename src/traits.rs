// traits.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{AreaBound, Cells, Pos};
use crate::widget::Border;
use crate::Result;

/// User interface component
///
/// Widgets are laid out into [GridArea]s, which are drawn onto a [Screen].
///
/// [GridArea]: layout/struct.GridArea.html
/// [Screen]: struct.Screen.html
pub trait Widget {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        AreaBound::default()
    }

    /// Get the border (if any)
    fn border(&self) -> Option<Border> {
        None
    }

    /// Draw the widget
    ///
    /// * `_cells`: Text cells to draw onto
    fn draw(&self, _cells: &mut Cells) -> Result<()> {
        // default implementation draws nothing
        Ok(())
    }

    /// Handle a focus event
    ///
    /// * `_fev`: The focus event
    ///
    /// ## Return
    ///
    /// If the event triggers a `Redraw` [Action], it is returned.
    fn focus(&self, _fev: FocusEvent) -> Option<Action> {
        None
    }

    /// Handle a mouse event
    ///
    /// * `_mev`: The mouse event
    /// * `_mods`: Pressed modifier keys
    /// * `_pos`: Position relative to top-left of widget
    ///
    /// ## Return
    ///
    /// If the event triggers an [Action], it is returned.
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
