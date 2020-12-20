// border.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, BBox, Cells};
use crate::text::Outline;
use crate::{Result, Widget};

/// Border widget
///
/// One or more edges are drawn around a bounding box.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Border {
    /// Style for left edge
    left: Option<Outline>,

    /// Style for right edge
    right: Option<Outline>,

    /// Style for top edge
    top: Option<Outline>,

    /// Style for bottom edge
    bottom: Option<Outline>,
}

impl Border {
    /// Set outline for all edges
    pub fn with_outline(mut self, outline: Outline) -> Self {
        self.left = Some(outline);
        self.right = Some(outline);
        self.top = Some(outline);
        self.bottom = Some(outline);
        self
    }

    /// Set outline for left edge
    pub fn with_left(mut self, outline: Option<Outline>) -> Self {
        self.left = outline;
        self
    }

    /// Set outline for right edge
    pub fn with_right(mut self, outline: Option<Outline>) -> Self {
        self.right = outline;
        self
    }

    /// Set outline for top edge
    pub fn with_top(mut self, outline: Option<Outline>) -> Self {
        self.top = outline;
        self
    }

    /// Set outline for bottom edge
    pub fn with_bottom(mut self, outline: Option<Outline>) -> Self {
        self.bottom = outline;
        self
    }

    /// Get the total width in cells (left and right edges)
    pub fn width(self) -> u16 {
        let mut width = 0;
        if self.left.is_some() {
            width += 1;
        }
        if self.right.is_some() {
            width += 1;
        }
        width
    }

    /// Get the total height in cells (top and bottom edges)
    pub fn height(self) -> u16 {
        let mut height = 0;
        if self.top.is_some() {
            height += 1;
        }
        if self.bottom.is_some() {
            height += 1;
        }
        height
    }

    /// Get the bbox inside the border
    pub fn inset(self, mut bbox: BBox) -> BBox {
        let trim = 1;
        if self.left.is_some() {
            bbox = bbox.trim_left(trim);
        }
        if self.right.is_some() {
            bbox = bbox.trim_right(trim);
        }
        if self.top.is_some() {
            bbox = bbox.trim_top(trim);
        }
        if self.bottom.is_some() {
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

    /// Render the widget
    fn render(&self, cells: &mut Cells) -> Result<()> {
        let primary = cells.theme().primary();
        let background = cells.theme().background();
        cells.set_foreground_color(primary)?;
        cells.set_background_color(background)?;
        let width = cells.width();
        let height = cells.height();
        if width == 0 || height == 0 {
            return Ok(());
        }
        let inset = self.inset(BBox::new(0, 0, width, height));
        let mut row = 0;
        if let Some(top) = self.top {
            cells.move_to(0, 0)?;
            if let Some(left) = self.left {
                cells.print_char(top.top_left(left))?;
            }
            for _ in 0..inset.width() {
                cells.print_char(top.top())?;
            }
            if let Some(right) = self.right {
                cells.print_char(top.top_right(right))?;
            }
            row += 1;
        }
        for _ in 0..inset.height() {
            if let Some(left) = self.left {
                cells.move_to(0, row)?;
                cells.print_char(left.left())?;
            }
            if let Some(right) = self.right {
                if self.left.is_some() {
                    cells.move_right(inset.width())?;
                } else {
                    cells.move_to(inset.width(), row)?;
                }
                cells.print_char(right.right())?;
            }
            row += 1;
        }
        if let Some(bottom) = self.bottom {
            cells.move_to(0, row)?;
            if let Some(left) = self.left {
                cells.print_char(bottom.bottom_left(left))?;
            }
            for _ in 0..inset.width() {
                cells.print_char(bottom.bottom())?;
            }
            if let Some(right) = self.right {
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
        let bdr = Border::default().with_outline(Outline::default());
        let bbox = BBox::new(0, 0, 10, 10);
        assert_eq!(bdr.inset(bbox), BBox::new(1, 1, 8, 8));
    }
}
