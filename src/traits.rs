// traits.rs
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{Cells, Dim, LengthBound, Pos};
use crate::text::{StyleGroup, Theme, WidgetGroup};
use crate::widget::{Border, Button, ScrollView};
use crate::Result;

/// User interface component
///
/// Widgets are laid out into [GridArea]s, which are drawn onto a [Screen].
///
/// [GridArea]: layout/struct.GridArea.html
/// [Screen]: struct.Screen.html
pub trait Widget {
    /// Get the widget group
    fn widget_group(&self) -> WidgetGroup {
        WidgetGroup::Normal
    }

    /// Get the style group
    fn style_group(&self) -> StyleGroup {
        StyleGroup::Enabled
    }

    /// Get the width bounds
    fn width_bounds(&self, _theme: &Theme) -> LengthBound {
        LengthBound::default()
    }

    /// Get the height bounds
    fn height_bounds(&self, _theme: &Theme, _width: u16) -> LengthBound {
        LengthBound::default()
    }

    /// Draw the widget
    ///
    /// * `_cells`: Text cells to draw onto
    /// * `_offset`: Offset from top-left position within widget
    fn draw(&self, _cells: &mut Cells, _offset: Pos) -> Result<()> {
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
    /// * `_dim`: Dimensions of widget
    /// * `_pos`: Position relative to top-left of widget
    ///
    /// ## Return
    ///
    /// If the event triggers an [Action], it is returned.
    fn mouse_event(
        &self,
        _mev: MouseEvent,
        _mods: ModKeys,
        _dim: Dim,
        _pos: Pos,
    ) -> Option<Action> {
        // ignore by default
        None
    }

    /// Wrap the widget with a border
    fn into_border(self) -> Border<Self>
    where
        Self: Sized,
    {
        Border::new(self)
    }

    /// Wrap the widget with a button
    fn into_button(self) -> Border<Button<Self>>
    where
        Self: Sized,
    {
        Border::new(Button::new(self))
    }

    /// Wrap the widget with a scroll view
    fn into_scroll_view(self) -> ScrollView<Self>
    where
        Self: Sized,
    {
        ScrollView::new(self)
    }
}
