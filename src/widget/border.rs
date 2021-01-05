// border.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{AreaBound, BBox, Cells, Pos};
use crate::text::{Outline, StyleGroup, Theme};
use crate::{Result, Widget};

/// Border height
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BorderHeight {
    /// Raised border
    Raised,
    /// Lowered border
    Lowered,
}

/// Border style
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BorderStyle {
    /// Simple border
    Simple(Outline),
    /// Beveled appearance
    Bevel(Outline, BorderHeight),
    /// Drop shadow
    Shadow(Outline, BorderHeight),
}

/// Border widget wrapper
///
/// One or more outline edges are drawn around a wrapped widget.
pub struct Border<W: Widget> {
    /// Wrapped widget
    wrapped: W,
    /// Border style
    border_style: Option<BorderStyle>,
}

impl BorderStyle {
    /// Get the fallback outline
    fn outline(self) -> Option<Outline> {
        match self {
            BorderStyle::Simple(outline) => Some(outline),
            BorderStyle::Bevel(outline, _) => Some(outline),
            _ => None,
        }
    }

    /// Get the left border outline
    fn outline_left(self) -> Option<Outline> {
        self.outline()
    }

    /// Get the right border outline
    fn outline_right(self) -> Option<Outline> {
        match self {
            BorderStyle::Shadow(outline, _) => Some(outline),
            _ => self.outline(),
        }
    }

    /// Get the top border outline
    fn outline_top(self) -> Option<Outline> {
        self.outline()
    }

    /// Get the bottom border outline
    fn outline_bottom(self) -> Option<Outline> {
        match self {
            BorderStyle::Shadow(outline, _) => Some(outline),
            _ => self.outline(),
        }
    }

    /// Get the left style group
    fn group_left(self) -> StyleGroup {
        use BorderHeight::*;
        use BorderStyle::*;
        match self {
            Bevel(_, Raised) | Shadow(_, Raised) => StyleGroup::LightShadow,
            Bevel(_, Lowered) | Shadow(_, Lowered) => StyleGroup::DarkShadow,
            _ => StyleGroup::Enabled,
        }
    }

    /// Get the right style group
    fn group_right(self) -> StyleGroup {
        use BorderHeight::*;
        use BorderStyle::*;
        match self {
            Bevel(_, Raised) | Shadow(_, Raised) => StyleGroup::DarkShadow,
            Bevel(_, Lowered) | Shadow(_, Lowered) => StyleGroup::LightShadow,
            _ => StyleGroup::Enabled,
        }
    }

    /// Get the top style group
    fn group_top(self) -> StyleGroup {
        use BorderHeight::*;
        use BorderStyle::*;
        match self {
            Bevel(_, Raised) | Shadow(_, Raised) => StyleGroup::LightShadow,
            Bevel(_, Lowered) | Shadow(_, Lowered) => StyleGroup::DarkShadow,
            _ => StyleGroup::Enabled,
        }
    }

    /// Get the bottom style group
    fn group_bottom(self) -> StyleGroup {
        use BorderHeight::*;
        use BorderStyle::*;
        match self {
            Bevel(_, Raised) | Shadow(_, Raised) => StyleGroup::DarkShadow,
            Bevel(_, Lowered) | Shadow(_, Lowered) => StyleGroup::LightShadow,
            _ => StyleGroup::Enabled,
        }
    }

    /// Get the total width in cells (left and right edges)
    pub fn width(self) -> u16 {
        use BorderStyle::*;
        match self {
            Simple(_) | Bevel(_, _) => 2,
            Shadow(_, _) => 1,
        }
    }

    /// Get the total height in cells (top and bottom edges)
    pub fn height(self) -> u16 {
        use BorderStyle::*;
        match self {
            Simple(_) | Bevel(_, _) => 2,
            Shadow(_, _) => 1,
        }
    }

    /// Get the bbox inside the border
    fn inset(self, mut bbox: BBox) -> BBox {
        let trim = 1;
        if self.outline_left().is_some() {
            bbox = bbox.trim_left(trim);
        }
        if self.outline_right().is_some() {
            bbox = bbox.trim_right(trim);
        }
        if self.outline_top().is_some() {
            bbox = bbox.trim_top(trim);
        }
        if self.outline_bottom().is_some() {
            bbox = bbox.trim_bottom(trim);
        }
        bbox
    }
}

impl<W: Widget> Border<W> {
    /// Create a new border
    pub fn new(wrapped: W) -> Self {
        let border_style = None;
        Self {
            wrapped,
            border_style,
        }
    }

    /// Get the wrapped widget
    pub fn wrapped(&self) -> &W {
        &self.wrapped
    }

    /// Set the border style
    ///
    /// - `border_style`: Style to override [Theme], or `None`
    pub fn set_border_style(&mut self, border_style: Option<BorderStyle>) {
        self.border_style = border_style;
    }

    /// Get the border style
    fn border_style(&self, theme: &Theme) -> BorderStyle {
        self.border_style
            .unwrap_or_else(|| theme.border_style(self.wrapped.style_group()))
    }
}

impl<W: Widget> Widget for Border<W> {
    /// Get the area bounds
    fn bounds(&self, theme: &Theme) -> AreaBound {
        let bounds = self.wrapped.bounds(theme);
        let bs = self.border_style(theme);
        let cols = bs.width();
        let rows = bs.height();
        bounds + AreaBound::default().with_columns(cols..).with_rows(rows..)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let width = cells.width();
        let height = cells.height();
        if width == 0 || height == 0 {
            return Ok(());
        }
        let theme = cells.theme();
        let bs = self.border_style(theme);
        let disabled = if self.wrapped.style_group() == StyleGroup::Disabled {
            Some(StyleGroup::Disabled)
        } else {
            None
        };
        let style_top = theme.style(disabled.unwrap_or(bs.group_top()));
        let style_left = theme.style(disabled.unwrap_or(bs.group_left()));
        let style_right = theme.style(disabled.unwrap_or(bs.group_right()));
        let style_bottom = theme.style(disabled.unwrap_or(bs.group_bottom()));
        let inset = bs.inset(BBox::new(0, 0, width, height));
        let mut row = 0;
        if let Some(top) = bs.outline_top() {
            cells.set_style(style_top)?;
            cells.move_to(0, 0)?;
            if let Some(left) = bs.outline_left() {
                cells.print_char(top.top_left(left))?;
            }
            for _ in 0..inset.width() {
                cells.print_char(top.top())?;
            }
            if let Some(right) = bs.outline_right() {
                cells.set_style(style_right)?;
                cells.print_char(top.top_right(right))?;
            }
            row += 1;
        }
        for _ in 0..inset.height() {
            if let Some(left) = bs.outline_left() {
                cells.set_style(style_left)?;
                cells.move_to(0, row)?;
                cells.print_char(left.left())?;
            }
            if let Some(right) = bs.outline_right() {
                if bs.outline_left().is_some() {
                    cells.move_right(inset.width())?;
                } else {
                    cells.move_to(inset.width(), row)?;
                }
                if bs.outline_right().is_some() {
                    cells.set_style(style_right)?;
                }
                cells.print_char(right.right())?;
            }
            row += 1;
        }
        if let Some(bottom) = bs.outline_bottom() {
            if bs.outline_left().is_some() {
                cells.set_style(style_left)?;
            } else {
                cells.set_style(style_bottom)?;
            }
            cells.move_to(0, row)?;
            if let Some(left) = bs.outline_left() {
                cells.print_char(bottom.bottom_left(left))?;
            }
            cells.set_style(style_bottom)?;
            for _ in 0..inset.width() {
                cells.print_char(bottom.bottom())?;
            }
            if let Some(right) = bs.outline_right() {
                cells.print_char(bottom.bottom_right(right))?;
            }
        }
        cells.clip(inset);
        self.wrapped.draw(cells)
    }

    /// Handle focus event
    fn focus(&self, fev: FocusEvent) -> Option<Action> {
        self.wrapped.focus(fev)
    }

    /// Handle mouse events
    fn mouse_event(
        &self,
        mev: MouseEvent,
        mods: ModKeys,
        pos: Pos,
    ) -> Option<Action> {
        self.wrapped.mouse_event(mev, mods, pos)
    }
}
