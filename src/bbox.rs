// bbox.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use bitflags::bitflags;

bitflags! {
    /// Edge of a widget or bounding box
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

/// Text cell position
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pos {
    /// Column relative to left edge
    pub col: u16,
    /// Row relative to top edge
    pub row: u16,
}

/// Text cell dimensions
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Dim {
    /// Width in text cells
    pub width: u16,
    /// Height in text cells
    pub height: u16,
}

/// Bounding box of text cells
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BBox {
    /// Position of top-left cell
    pos: Pos,
    /// Dimensions in text cells
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

    /// Check if the dimension is empty
    pub fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}

impl BBox {
    /// Create a new bounding box
    pub fn new(col: u16, row: u16, width: u16, height: u16) -> Self {
        let pos = Pos::new(col, row);
        let dim = Dim::new(width, height);
        BBox { pos, dim }
    }

    /// Get the left column (inclusive)
    pub fn left(self) -> u16 {
        self.pos.col
    }

    /// Get the width
    pub fn width(self) -> u16 {
        self.dim.width
    }

    /// Get the right column (exclusive)
    pub fn right(self) -> u16 {
        self.left() + self.width()
    }

    /// Get the top row (inclusive)
    pub fn top(self) -> u16 {
        self.pos.row
    }

    /// Get the height
    pub fn height(self) -> u16 {
        self.dim.height
    }

    /// Get the bottom row (exclusive)
    pub fn bottom(self) -> u16 {
        self.top() + self.height()
    }

    /// Check if the bounding box is empty
    pub fn is_empty(self) -> bool {
        self.dim.is_empty()
    }

    /// Clip with another bounding box
    pub fn clip(self, rhs: Self) -> Self {
        let col = self.left().max(rhs.left());
        let row = self.top().max(rhs.top());
        let right = self.right().min(rhs.right());
        let bottom = self.bottom().min(rhs.bottom());
        let width = if right > col { right - col } else { 0 };
        let height = if bottom > row { bottom - row } else { 0 };
        BBox::new(col, row, width, height)
    }

    /// Trim cells from the given edges
    pub fn trim(self, edge: Edge, cells: u16) -> Self {
        let mut bbox = self;
        if edge.contains(Edge::LEFT) {
            bbox.trim_left(cells);
        }
        if edge.contains(Edge::RIGHT) {
            bbox.trim_right(cells);
        }
        if edge.contains(Edge::TOP) {
            bbox.trim_top(cells);
        }
        if edge.contains(Edge::BOTTOM) {
            bbox.trim_bottom(cells);
        }
        bbox
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
    fn bbox_trim() {
        let bbox = BBox::new(0, 0, 5, 7);
        assert_eq!(bbox.trim(Edge::LEFT, 1), BBox::new(1, 0, 4, 7));
    }
}
