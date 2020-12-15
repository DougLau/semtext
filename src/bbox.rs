// bbox.rs
//
// Copyright (c) 2020  Douglas P Lau
//

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
        Self { pos, dim }
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

    /// Trim cells from left edge
    pub fn trim_left(mut self, trim: u16) -> Self {
        let trim = self.width().min(trim);
        self.pos.col += trim;
        self.dim.width -= trim;
        self
    }

    /// Trim cells from right edge
    pub fn trim_right(mut self, trim: u16) -> Self {
        let trim = self.width().min(trim);
        self.dim.width -= trim;
        self
    }

    /// Trim cells from top edge
    pub fn trim_top(mut self, trim: u16) -> Self {
        let trim = self.height().min(trim);
        self.pos.row += trim;
        self.dim.height -= trim;
        self
    }

    /// Trim cells from bottom edge
    pub fn trim_bottom(mut self, trim: u16) -> Self {
        let trim = self.height().min(trim);
        self.dim.height -= trim;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bbox_trim() {
        let bbox = BBox::new(0, 0, 5, 7);
        assert_eq!(bbox.trim_left(1), BBox::new(1, 0, 4, 7));
        assert_eq!(bbox.trim_right(1), BBox::new(0, 0, 4, 7));
        assert_eq!(bbox.trim_top(1), BBox::new(0, 1, 5, 6));
        assert_eq!(bbox.trim_bottom(1), BBox::new(0, 0, 5, 6));
    }
}
