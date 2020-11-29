// area.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use bitflags::bitflags;

bitflags! {
    /// Area edges
    #[derive(Default)]
    pub struct Edge: u8 {
        const NONE = 0x00;
        const TOP = 0x01;
        const BOTTOM = 0x02;
        const LEFT = 0x04;
        const RIGHT = 0x08;
        const TOP_LEFT = Self::TOP.bits | Self::LEFT.bits;
        const TOP_RIGHT = Self::TOP.bits | Self::RIGHT.bits;
        const BOTTOM_LEFT = Self::BOTTOM.bits | Self::LEFT.bits;
        const BOTTOM_RIGHT = Self::BOTTOM.bits | Self::RIGHT.bits;
        const TOP_BOTTOM = Self::TOP.bits | Self::BOTTOM.bits;
        const LEFT_RIGHT = Self::LEFT.bits | Self::RIGHT.bits;
        const ALL = Self::TOP_BOTTOM.bits | Self::LEFT_RIGHT.bits;
    }
}

/// Text grid cell position
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    /// Column relative to left edge of grid
    pub col: u16,
    /// Row relative to top edge of grid
    pub row: u16,
}

/// Text grid cell dimensions
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dim {
    /// Width in text cells
    pub width: u16,
    /// Height in text cells
    pub height: u16,
}

/// Rectangular area of text grid cells
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Area {
    /// Position of top-left cell
    pos: Pos,
    /// Dimensions in grid cells
    dim: Dim,
}

impl Pos {
    /// Create a new position
    pub fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

impl Dim {
    /// Create a new cell dimension
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Area {
    /// Create a new area
    pub fn new(col: u16, row: u16, width: u16, height: u16) -> Self {
        let pos = Pos::new(col, row);
        let dim = Dim::new(width, height);
        Area { pos, dim }
    }

    /// Get the left column
    pub fn col(self) -> u16 {
        self.pos.col
    }

    /// Get the top row
    pub fn row(self) -> u16 {
        self.pos.row
    }

    /// Get the width
    pub fn width(self) -> u16 {
        self.dim.width
    }

    /// Get the height
    pub fn height(self) -> u16 {
        self.dim.height
    }

    /// Split into two areas starting from a given edge
    pub fn split(self, edge: Edge, cells: u16) -> (Self, Self) {
        match edge {
            Edge::LEFT => self.split_left(cells),
            Edge::RIGHT => self.split_right(cells),
            Edge::LEFT_RIGHT => self.split_horiz(),
            Edge::TOP => self.split_top(cells),
            Edge::BOTTOM => self.split_bottom(cells),
            Edge::TOP_BOTTOM => self.split_vert(),
            _ => panic!("Invalid Edges"),
        }
    }

    /// Split from left edge
    fn split_left(self, width: u16) -> (Self, Self) {
        let mut left = self;
        left.dim.width = self.width().min(width);
        let mut right = self;
        right.pos.col = self.col() + left.width();
        right.dim.width = self.width() - left.width();
        (left, right)
    }

    /// Split from right edge
    fn split_right(self, width: u16) -> (Self, Self) {
        let mut right = self;
        right.dim.width = self.width().min(width);
        right.pos.col = self.col() + self.width() - right.width();
        let mut left = self;
        left.dim.width = self.width() - right.width();
        (right, left)
    }

    /// Split horizontally
    fn split_horiz(self) -> (Self, Self) {
        self.split_left(self.width() / 2)
    }

    /// Split from top edge
    fn split_top(self, height: u16) -> (Self, Self) {
        let mut top = self;
        top.dim.height = self.height().min(height);
        let mut bottom = self;
        bottom.dim.height = self.height() - top.height();
        bottom.pos.row = self.row() + top.height();
        (top, bottom)
    }

    /// Split from bottom edge
    fn split_bottom(self, height: u16) -> (Self, Self) {
        let mut bottom = self;
        bottom.dim.height = self.height().min(height);
        bottom.pos.row = self.row() + self.height() - bottom.height();
        let mut top = self;
        top.dim.height = self.height() - bottom.height();
        (bottom, top)
    }

    /// Split vertically
    fn split_vert(self) -> (Self, Self) {
        self.split_top(self.height() / 2)
    }

    /// Trim cells from the given edges
    pub fn trim(self, edge: Edge, cells: u16) -> Self {
        let mut area = self;
        if edge.contains(Edge::LEFT) {
            area.trim_left(cells);
        }
        if edge.contains(Edge::RIGHT) {
            area.trim_right(cells);
        }
        if edge.contains(Edge::TOP) {
            area.trim_top(cells);
        }
        if edge.contains(Edge::BOTTOM) {
            area.trim_bottom(cells);
        }
        area
    }

    /// Trim cells from left edge
    fn trim_left(&mut self, cells: u16) {
        let cells = self.width().min(cells);
        self.pos.col += cells;
        self.dim.width -= cells;
    }

    /// Trim cells from right edge
    fn trim_right(&mut self, cells: u16) {
        let cells = self.width().min(cells);
        self.dim.width -= cells;
    }

    /// Trim cells from top edge
    fn trim_top(&mut self, cells: u16) {
        let cells = self.height().min(cells);
        self.pos.row += cells;
        self.dim.height -= cells;
    }

    /// Trim cells from bottom edge
    fn trim_bottom(&mut self, cells: u16) {
        let cells = self.height().min(cells);
        self.dim.height -= cells;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn area_trim() {
        let area = Area::new(0, 0, 5, 7);
        assert_eq!(area.trim(Edge::LEFT, 1), Area::new(1, 0, 4, 7));
    }
}
