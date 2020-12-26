// border.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, BBox, Cells};
use crate::text::{Outline, Style, Theme};
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
    /// Beveled appearance
    Bevel(BorderHeight),
    /// Something else
    ShadowRightBottom,
}

/// Border widget
///
/// One or more edges are drawn around a bounding box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Border {
    bdr_style: BorderStyle,
}

impl BorderStyle {
    /// Get the left border outline
    pub fn outline_left(self) -> Option<Outline> {
        use BorderStyle::*;
        match self {
            Bevel(_) => Some(Outline::Solid),
            ShadowRightBottom => None,
        }
    }

    /// Get the right border outline
    pub fn outline_right(self) -> Option<Outline> {
        use BorderStyle::*;
        match self {
            Bevel(_) => Some(Outline::Solid),
            ShadowRightBottom => Some(Outline::Block),
        }
    }

    /// Get the top border outline
    pub fn outline_top(self) -> Option<Outline> {
        use BorderStyle::*;
        match self {
            Bevel(_) => Some(Outline::Solid),
            ShadowRightBottom => None,
        }
    }

    /// Get the bottom border outline
    pub fn outline_bottom(self) -> Option<Outline> {
        use BorderStyle::*;
        match self {
            Bevel(_) => Some(Outline::Solid),
            ShadowRightBottom => Some(Outline::Block),
        }
    }

    /// Get the left style
    pub fn style_left(self, theme: &Theme) -> Option<Style> {
        use BorderStyle::*;
        match self {
            Bevel(BorderHeight::Raised) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.light_shadow),
            ),
            Bevel(BorderHeight::Lowered) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.dark_shadow),
            ),
            ShadowRightBottom => None,
        }
    }

    /// Get the right style
    pub fn style_right(self, theme: &Theme) -> Option<Style> {
        use BorderStyle::*;
        match self {
            Bevel(BorderHeight::Raised) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.dark_shadow),
            ),
            Bevel(BorderHeight::Lowered) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.light_shadow),
            ),
            ShadowRightBottom => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.dark_shadow),
            ),
        }
    }

    /// Get the top style
    pub fn style_top(self, theme: &Theme) -> Option<Style> {
        use BorderStyle::*;
        match self {
            Bevel(BorderHeight::Raised) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.light_shadow),
            ),
            Bevel(BorderHeight::Lowered) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.dark_shadow),
            ),
            ShadowRightBottom => None,
        }
    }

    /// Get the bottom style
    pub fn style_bottom(self, theme: &Theme) -> Option<Style> {
        use BorderStyle::*;
        match self {
            Bevel(BorderHeight::Raised) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.dark_shadow),
            ),
            Bevel(BorderHeight::Lowered) => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.light_shadow),
            ),
            ShadowRightBottom => Some(
                Style::default()
                    .with_background(theme.background)
                    .with_foreground(theme.dark_shadow),
            ),
        }
    }

    /// Get the total width in cells (left and right edges)
    pub fn width(self) -> u16 {
        use BorderStyle::*;
        match self {
            Bevel(_) => 2,
            ShadowRightBottom => 1,
        }
    }

    /// Get the total height in cells (top and bottom edges)
    pub fn height(self) -> u16 {
        use BorderStyle::*;
        match self {
            Bevel(_) => 2,
            ShadowRightBottom => 1,
        }
    }
}

impl Border {
    /// Create a new border
    pub fn new(bdr_style: BorderStyle) -> Self {
        Self { bdr_style }
    }

    /// Get the total width in cells (left and right edges)
    pub fn width(self) -> u16 {
        self.bdr_style.width()
    }

    /// Get the total height in cells (top and bottom edges)
    pub fn height(self) -> u16 {
        self.bdr_style.height()
    }

    /// Get the bbox inside the border
    pub fn inset(self, mut bbox: BBox) -> BBox {
        let trim = 1;
        let bs = self.bdr_style;
        if bs.outline_left().is_some() {
            bbox = bbox.trim_left(trim);
        }
        if bs.outline_right().is_some() {
            bbox = bbox.trim_right(trim);
        }
        if bs.outline_top().is_some() {
            bbox = bbox.trim_top(trim);
        }
        if bs.outline_bottom().is_some() {
            bbox = bbox.trim_bottom(trim);
        }
        bbox
    }
}

impl Widget for Border {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        let cols = self.width();
        let rows = self.height();
        AreaBound::default().with_columns(cols..).with_rows(rows..)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let width = cells.width();
        let height = cells.height();
        if width == 0 || height == 0 {
            return Ok(());
        }
        let theme = cells.theme();
        let bs = self.bdr_style;
        let style_top = bs.style_top(&theme);
        let style_left = bs.style_left(&theme);
        let style_right = bs.style_right(&theme);
        let style_bottom = bs.style_bottom(&theme);
        let inset = self.inset(BBox::new(0, 0, width, height));
        let mut row = 0;
        if let Some(top) = bs.outline_top() {
            if let Some(style) = style_top {
                cells.set_style(style)?;
            }
            cells.move_to(0, 0)?;
            if let Some(left) = bs.outline_left() {
                cells.print_char(top.top_left(left))?;
            }
            for _ in 0..inset.width() {
                cells.print_char(top.top())?;
            }
            if let Some(right) = bs.outline_right() {
                cells.print_char(top.top_right(right))?;
            }
            row += 1;
        }
        for _ in 0..inset.height() {
            if let Some(left) = bs.outline_left() {
                if let Some(style) = style_left {
                    cells.set_style(style)?;
                }
                cells.move_to(0, row)?;
                cells.print_char(left.left())?;
            }
            if let Some(right) = bs.outline_right() {
                if bs.outline_left().is_some() {
                    cells.move_right(inset.width())?;
                } else {
                    cells.move_to(inset.width(), row)?;
                }
                if let Some(style) = style_right {
                    cells.set_style(style)?;
                }
                cells.print_char(right.right())?;
            }
            row += 1;
        }
        if let Some(bottom) = bs.outline_bottom() {
            if let Some(style) = style_left {
                cells.set_style(style)?;
            } else if let Some(style) = style_bottom {
                cells.set_style(style)?;
            }
            cells.move_to(0, row)?;
            if let Some(left) = bs.outline_left() {
                cells.print_char(bottom.bottom_left(left))?;
            }
            if let Some(style) = style_bottom {
                cells.set_style(style)?;
            }
            for _ in 0..inset.width() {
                cells.print_char(bottom.bottom())?;
            }
            if let Some(right) = bs.outline_right() {
                cells.print_char(bottom.bottom_right(right))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn border() {
        let bdr = Border::new(BorderStyle::Bevel(BorderHeight::Raised));
        let bbox = BBox::new(0, 0, 10, 10);
        assert_eq!(bdr.inset(bbox), BBox::new(1, 1, 8, 8));
    }
}
