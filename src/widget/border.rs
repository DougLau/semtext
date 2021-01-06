// border.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{AreaBound, BBox, Cells, Pos};
use crate::text::{Outline, StyleGroup, Theme};
use crate::{Result, Widget};

/// Border elevation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Elevation {
    /// Unknown elevation
    Unknown,
    /// Leveled elevation
    Leveled,
    /// Raised elevation
    Raised,
    /// Lowered elevation
    Lowered,
}

/// Border edge
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Edge {
    /// Left edge of border
    Left,
    /// Top edge of border
    Top,
    /// Right edge of border
    Right,
    /// Bottom edge of border
    Bottom,
}

/// Border style
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BorderStyle {
    /// Simple border
    Simple(Outline),
    /// Beveled appearance
    Bevel(Outline),
    /// Drop shadow
    Shadow(Outline),
    /// Custom outline
    ///
    /// Edges: `Left`, `Top`, `Right`, `Bottom`
    Custom(
        Option<Outline>,
        Option<Outline>,
        Option<Outline>,
        Option<Outline>,
    ),
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

impl Elevation {
    /// Get border elevation from a wrapped style group
    fn from_style_group(bdr_style: BorderStyle, group: StyleGroup) -> Self {
        match (bdr_style, group) {
            (_, StyleGroup::Disabled) => Elevation::Unknown,
            (BorderStyle::Simple(_), _)
            | (BorderStyle::Custom(_, _, _, _), _) => Elevation::Leveled,
            (_, StyleGroup::Interacted) => Elevation::Lowered,
            _ => Elevation::Raised,
        }
    }

    /// Get the style group for an edge
    fn edge_group(self, edge: Edge) -> StyleGroup {
        match (self, edge) {
            (Elevation::Unknown, _) => StyleGroup::Disabled,
            (Elevation::Leveled, _) => StyleGroup::Primary,
            (Elevation::Raised, Edge::Left) => StyleGroup::LightShadow,
            (Elevation::Raised, Edge::Top) => StyleGroup::LightShadow,
            (Elevation::Raised, Edge::Right) => StyleGroup::DarkShadow,
            (Elevation::Raised, Edge::Bottom) => StyleGroup::DarkShadow,
            (Elevation::Lowered, Edge::Left) => StyleGroup::DarkShadow,
            (Elevation::Lowered, Edge::Top) => StyleGroup::DarkShadow,
            (Elevation::Lowered, Edge::Right) => StyleGroup::LightShadow,
            (Elevation::Lowered, Edge::Bottom) => StyleGroup::LightShadow,
        }
    }
}

impl BorderStyle {
    /// Get outline for an edge
    fn outline(self, edge: Edge) -> Option<Outline> {
        use BorderStyle::*;
        match (self, edge) {
            (Simple(outline), _) => Some(outline),
            (Bevel(outline), _) => Some(outline),
            (Shadow(_), Edge::Left) => None,
            (Shadow(_), Edge::Top) => None,
            (Shadow(outline), _) => Some(outline),
            (Custom(outline, _, _, _), Edge::Left) => outline,
            (Custom(_, outline, _, _), Edge::Top) => outline,
            (Custom(_, _, outline, _), Edge::Right) => outline,
            (Custom(_, _, _, outline), Edge::Bottom) => outline,
        }
    }

    /// Get the total width in cells (left and right edges)
    pub fn width(self) -> u16 {
        match self {
            BorderStyle::Shadow(_) => 1,
            _ => 2,
        }
    }

    /// Get the total height in cells (top and bottom edges)
    pub fn height(self) -> u16 {
        match self {
            BorderStyle::Shadow(_) => 1,
            _ => 2,
        }
    }

    /// Get the bbox inside the border
    fn inset(self, mut bbox: BBox) -> BBox {
        let trim = 1;
        if self.outline(Edge::Left).is_some() {
            bbox = bbox.trim_left(trim);
        }
        if self.outline(Edge::Right).is_some() {
            bbox = bbox.trim_right(trim);
        }
        if self.outline(Edge::Top).is_some() {
            bbox = bbox.trim_top(trim);
        }
        if self.outline(Edge::Bottom).is_some() {
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
    pub fn with_border_style(
        mut self,
        border_style: Option<BorderStyle>,
    ) -> Self {
        self.border_style = border_style;
        self
    }

    /// Get the border style
    fn border_style(&self, theme: &Theme) -> BorderStyle {
        self.border_style
            .unwrap_or_else(|| theme.border_style(self.wrapped.widget_group()))
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
        let group = self.wrapped.style_group();
        let elevation = Elevation::from_style_group(bs, group);
        let style_top = theme.style(elevation.edge_group(Edge::Top));
        let style_left = theme.style(elevation.edge_group(Edge::Left));
        let style_right = theme.style(elevation.edge_group(Edge::Right));
        let style_bottom = theme.style(elevation.edge_group(Edge::Bottom));
        let inset = bs.inset(BBox::new(0, 0, width, height));
        let mut row = 0;
        if let Some(top) = bs.outline(Edge::Top) {
            cells.set_style(style_top)?;
            cells.move_to(0, 0)?;
            if let Some(left) = bs.outline(Edge::Left) {
                cells.print_char(top.top_left(left))?;
            }
            for _ in 0..inset.width() {
                cells.print_char(top.top())?;
            }
            if let Some(right) = bs.outline(Edge::Right) {
                cells.set_style(style_right)?;
                cells.print_char(top.top_right(right))?;
            }
            row += 1;
        }
        for _ in 0..inset.height() {
            if let Some(left) = bs.outline(Edge::Left) {
                cells.set_style(style_left)?;
                cells.move_to(0, row)?;
                cells.print_char(left.left())?;
            }
            if let Some(right) = bs.outline(Edge::Right) {
                if bs.outline(Edge::Left).is_some() {
                    cells.move_right(inset.width())?;
                } else {
                    cells.move_to(inset.width(), row)?;
                }
                if bs.outline(Edge::Right).is_some() {
                    cells.set_style(style_right)?;
                }
                cells.print_char(right.right())?;
            }
            row += 1;
        }
        if let Some(bottom) = bs.outline(Edge::Bottom) {
            if bs.outline(Edge::Left).is_some() {
                cells.set_style(style_left)?;
            } else {
                cells.set_style(style_bottom)?;
            }
            cells.move_to(0, row)?;
            if let Some(left) = bs.outline(Edge::Left) {
                cells.print_char(bottom.bottom_left(left))?;
            }
            cells.set_style(style_bottom)?;
            for _ in 0..inset.width() {
                cells.print_char(bottom.bottom())?;
            }
            if let Some(right) = bs.outline(Edge::Right) {
                cells.print_char(bottom.bottom_right(right))?;
            }
        }
        cells.clip(inset);
        // Set style for wrapped widget draw
        let style = cells.theme().style(group);
        cells.set_style(style)?;
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
