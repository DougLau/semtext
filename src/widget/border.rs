// border.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::widget::LineStyle;
use crate::{AreaBound, BBox, Cells, Edge, Result, Widget};

/// Border widget
///
/// One or more edges are drawn around a bounding box.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Border {
    /// Normal edges
    edges: Edge,

    /// Accented edges
    accents: Edge,

    /// Line style
    line_style: LineStyle,
}

impl Border {
    /// Set border edges
    pub fn with_edges(mut self, edges: Edge) -> Self {
        self.edges = edges;
        self.with_accents(self.accents)
    }

    /// Set border edge accents
    pub fn with_accents(mut self, accents: Edge) -> Self {
        self.accents = self.edges & accents;
        self
    }

    /// Set border line style
    pub fn with_line_style(mut self, line_style: LineStyle) -> Self {
        self.line_style = line_style;
        self
    }

    /// Get the total width in cells (left and right edges)
    pub fn width(self) -> u16 {
        let mut width = 0;
        if self.edges.contains(Edge::LEFT) {
            width += 1;
        }
        if self.edges.contains(Edge::RIGHT) {
            width += 1;
        }
        width
    }

    /// Get the total height in cells (top and bottom edges)
    pub fn height(self) -> u16 {
        let mut height = 0;
        if self.edges.contains(Edge::TOP) {
            height += 1;
        }
        if self.edges.contains(Edge::BOTTOM) {
            height += 1;
        }
        height
    }

    /// Get the bbox inside the border
    pub fn inset(self, bbox: BBox) -> BBox {
        bbox.trim(self.edges, 1)
    }

    /// Get character at top edge
    pub fn top(self) -> char {
        match (self.line_style, self.accents & Edge::TOP) {
            (LineStyle::Solid, Edge::TOP) => '━',
            (LineStyle::Double, Edge::TOP) => '═',
            (LineStyle::Tight, Edge::TOP) => '▄',
            (LineStyle::Tight, _) => '▁',
            (LineStyle::Dashed, Edge::TOP) => '╍',
            (LineStyle::Dashed, _) => '╌',
            (LineStyle::Block, Edge::TOP) => '█',
            (LineStyle::Block, _) => '▄',
            (LineStyle::OuterBlock, Edge::TOP) => '█',
            (LineStyle::OuterBlock, _) => '▀',
            _ => '─',
        }
    }

    /// Get character at bottom edge
    pub fn bottom(self) -> char {
        match (self.line_style, self.accents & Edge::BOTTOM) {
            (LineStyle::Solid, Edge::BOTTOM) => '━',
            (LineStyle::Double, Edge::BOTTOM) => '═',
            (LineStyle::Tight, Edge::BOTTOM) => '▀',
            (LineStyle::Tight, _) => '▔',
            (LineStyle::Dashed, Edge::BOTTOM) => '╍',
            (LineStyle::Dashed, _) => '╌',
            (LineStyle::Block, Edge::BOTTOM) => '█',
            (LineStyle::Block, _) => '▀',
            (LineStyle::OuterBlock, Edge::BOTTOM) => '█',
            (LineStyle::OuterBlock, _) => '▄',
            _ => '─',
        }
    }

    /// Get character at left edge
    pub fn left(self) -> char {
        match (self.line_style, self.accents & Edge::LEFT) {
            (LineStyle::Solid, Edge::LEFT) => '┃',
            (LineStyle::Double, Edge::LEFT) => '║',
            (LineStyle::Tight, Edge::LEFT) => '▐',
            (LineStyle::Tight, _) => '▕',
            (LineStyle::Dashed, Edge::LEFT) => '┇',
            (LineStyle::Dashed, _) => '┆',
            (LineStyle::Block, Edge::LEFT) => '█',
            (LineStyle::Block, _) => '▐',
            (LineStyle::OuterBlock, Edge::LEFT) => '█',
            (LineStyle::OuterBlock, _) => '▌',
            _ => '│',
        }
    }

    /// Get character at right edge
    pub fn right(self) -> char {
        match (self.line_style, self.accents & Edge::RIGHT) {
            (LineStyle::Solid, Edge::RIGHT) => '┃',
            (LineStyle::Double, Edge::RIGHT) => '║',
            (LineStyle::Tight, Edge::LEFT) => '▌',
            (LineStyle::Tight, _) => '▏',
            (LineStyle::Dashed, Edge::RIGHT) => '┇',
            (LineStyle::Dashed, _) => '┆',
            (LineStyle::Block, Edge::RIGHT) => '█',
            (LineStyle::Block, _) => '▌',
            (LineStyle::OuterBlock, Edge::RIGHT) => '█',
            (LineStyle::OuterBlock, _) => '▐',
            _ => '│',
        }
    }

    /// Get character at top-left corner
    pub fn top_left(self) -> char {
        match (self.line_style, self.accents & Edge::TOP_LEFT) {
            (LineStyle::Double, Edge::TOP) => '╒',
            (LineStyle::Double, Edge::LEFT) => '╓',
            (LineStyle::Double, Edge::TOP_LEFT) => '╔',
            (LineStyle::Tight, Edge::TOP_LEFT) => '▗',
            (LineStyle::Tight, _) => ' ',
            (LineStyle::Block, Edge::TOP) => '▐',
            (LineStyle::Block, Edge::LEFT) => '▄',
            (LineStyle::Block, Edge::TOP_LEFT) => '█',
            (LineStyle::Block, _) => '▗',
            (LineStyle::OuterBlock, Edge::TOP) => '█',
            (LineStyle::OuterBlock, Edge::LEFT) => '█',
            (LineStyle::OuterBlock, Edge::TOP_LEFT) => '█',
            (LineStyle::OuterBlock, _) => '▛',
            (_, Edge::TOP) => '┍',
            (_, Edge::LEFT) => '┎',
            (_, Edge::TOP_LEFT) => '┏',
            _ => '╭',
        }
    }

    /// Get character at top-right corner
    pub fn top_right(self) -> char {
        match (self.line_style, self.accents & Edge::TOP_RIGHT) {
            (LineStyle::Double, Edge::TOP) => '╕',
            (LineStyle::Double, Edge::RIGHT) => '╖',
            (LineStyle::Double, Edge::TOP_RIGHT) => '╗',
            (LineStyle::Tight, Edge::TOP_RIGHT) => '▖',
            (LineStyle::Tight, _) => ' ',
            (LineStyle::Block, Edge::TOP) => '▌',
            (LineStyle::Block, Edge::RIGHT) => '▄',
            (LineStyle::Block, Edge::TOP_RIGHT) => '█',
            (LineStyle::Block, _) => '▖',
            (LineStyle::OuterBlock, Edge::TOP) => '█',
            (LineStyle::OuterBlock, Edge::RIGHT) => '█',
            (LineStyle::OuterBlock, Edge::TOP_RIGHT) => '█',
            (LineStyle::OuterBlock, _) => '▜',
            (_, Edge::TOP) => '┑',
            (_, Edge::RIGHT) => '┒',
            (_, Edge::TOP_RIGHT) => '┓',
            _ => '╮',
        }
    }

    /// Get character at bottom-left corner
    pub fn bottom_left(self) -> char {
        match (self.line_style, self.accents & Edge::BOTTOM_LEFT) {
            (LineStyle::Double, Edge::BOTTOM) => '╘',
            (LineStyle::Double, Edge::LEFT) => '╙',
            (LineStyle::Double, Edge::BOTTOM_LEFT) => '╚',
            (LineStyle::Tight, Edge::BOTTOM_LEFT) => '▝',
            (LineStyle::Tight, _) => ' ',
            (LineStyle::Block, Edge::BOTTOM) => '▐',
            (LineStyle::Block, Edge::LEFT) => '▀',
            (LineStyle::Block, Edge::BOTTOM_LEFT) => '█',
            (LineStyle::Block, _) => '▝',
            (LineStyle::OuterBlock, Edge::BOTTOM) => '█',
            (LineStyle::OuterBlock, Edge::LEFT) => '█',
            (LineStyle::OuterBlock, Edge::BOTTOM_LEFT) => '█',
            (LineStyle::OuterBlock, _) => '▙',
            (_, Edge::BOTTOM) => '┕',
            (_, Edge::LEFT) => '┖',
            (_, Edge::BOTTOM_LEFT) => '┗',
            _ => '╰',
        }
    }

    /// Get character at bottom-right corner
    pub fn bottom_right(self) -> char {
        match (self.line_style, self.accents & Edge::BOTTOM_RIGHT) {
            (LineStyle::Double, Edge::BOTTOM) => '╛',
            (LineStyle::Double, Edge::RIGHT) => '╜',
            (LineStyle::Double, Edge::BOTTOM_RIGHT) => '╝',
            (LineStyle::Tight, Edge::BOTTOM_RIGHT) => '▘',
            (LineStyle::Tight, _) => ' ',
            (LineStyle::Block, Edge::BOTTOM) => '▌',
            (LineStyle::Block, Edge::RIGHT) => '▀',
            (LineStyle::Block, Edge::BOTTOM_RIGHT) => '█',
            (LineStyle::Block, _) => '▘',
            (LineStyle::OuterBlock, Edge::BOTTOM) => '█',
            (LineStyle::OuterBlock, Edge::RIGHT) => '█',
            (LineStyle::OuterBlock, Edge::BOTTOM_RIGHT) => '█',
            (LineStyle::OuterBlock, _) => '▟',
            (_, Edge::BOTTOM) => '┙',
            (_, Edge::RIGHT) => '┚',
            (_, Edge::BOTTOM_RIGHT) => '┛',
            _ => '╯',
        }
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
        let edges = self.edges;
        let mut row = 0;
        if edges.contains(Edge::TOP) {
            cells.move_to(0, 0)?;
            if edges.contains(Edge::LEFT) {
                cells.print_char(self.top_left())?;
            }
            for _ in 0..inset.width() {
                cells.print_char(self.top())?;
            }
            if edges.contains(Edge::RIGHT) {
                cells.print_char(self.top_right())?;
            }
            row += 1;
        }
        for _ in 0..inset.height() {
            if edges.contains(Edge::LEFT) {
                cells.move_to(0, row)?;
                cells.print_char(self.left())?;
            }
            if edges.contains(Edge::RIGHT) {
                cells.move_to(inset.width() + 1, row)?;
                cells.print_char(self.right())?;
            }
            row += 1;
        }
        if edges.contains(Edge::BOTTOM) {
            cells.move_to(0, row)?;
            if edges.contains(Edge::LEFT) {
                cells.print_char(self.bottom_left())?;
            }
            for _ in 0..inset.width() {
                cells.print_char(self.bottom())?;
            }
            if edges.contains(Edge::RIGHT) {
                cells.print_char(self.bottom_right())?;
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
        let bdr = Border::default().with_edges(Edge::ALL);
        let bbox = BBox::new(0, 0, 10, 10);
        assert_eq!(bdr.inset(bbox), BBox::new(1, 1, 8, 8));
    }
}
