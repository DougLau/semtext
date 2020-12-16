// line.rs
//
// Copyright (c) 2020  Douglas P Lau
//

/// Styles for border lines
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Line {
    /// Solid line
    ///
    /// ```text
    ///   ╭───╮
    ///   │   │
    ///   ╰───╯
    /// ```
    Solid,
    /// Thick solid line
    ///
    /// ```text
    ///   ┏━━━┓
    ///   ┃   ┃
    ///   ┗━━━┛
    /// ```
    Thick,
    /// Doubled solid line
    ///
    /// ```text
    ///   ╔═══╗
    ///   ║   ║
    ///   ╚═══╝
    /// ```
    Double,
    /// Dashed line
    ///
    /// ```text
    ///   ╭╌╌╌╮
    ///   ┆   ┆
    ///   ╰╌╌╌╯
    /// ```
    Dashed,
    /// Thick dashed line
    ///
    /// ```text
    ///   ┏╍╍╍┓
    ///   ┇   ┇
    ///   ┗╍╍╍┛
    /// ```
    DashedThick,
    /// Tightly packed line
    ///
    /// ```text
    ///    ▁▁▁
    ///   ▕   ▏
    ///    ▔▔▔
    /// ```
    Tight,
    /// Half block line
    ///
    /// ```text
    ///   ▗▄▄▄▖
    ///   ▐   ▌
    ///   ▝▀▀▀▘
    /// ```
    HalfInner,
    /// Full Block line
    ///
    /// ```text
    ///   █████
    ///   █   █
    ///   █████
    /// ```
    Block,
    /// Outer block line
    ///
    /// ```text
    ///   ▛▀▀▀▜
    ///   ▌   ▐
    ///   ▙▄▄▄▟
    /// ```
    HalfOuter,
}

impl Default for Line {
    fn default() -> Self {
        Line::Solid
    }
}

impl Line {
    /// Get character at top edge
    pub fn top(self) -> char {
        use Line::*;
        match self {
            Solid => '─',
            Thick => '━',
            Double => '═',
            Dashed => '╌',
            DashedThick => '╍',
            Tight => '▁',
            HalfInner => '▄',
            Block => '█',
            HalfOuter => '▀',
        }
    }

    /// Get character at left edge
    pub fn left(self) -> char {
        use Line::*;
        match self {
            Solid => '│',
            Thick => '┃',
            Double => '║',
            Dashed => '┆',
            DashedThick => '┇',
            Tight => '▕',
            Block => '█',
            HalfInner => '▐',
            HalfOuter => '▌',
        }
    }

    /// Get character at top-left corner
    pub fn top_left(self, left: Self) -> char {
        use Line::*;
        match (self, left) {
            (Solid, Solid) | (Solid, Dashed) => '╭',
            (Solid, Thick) | (Solid, DashedThick) => '┎',
            (Solid, Double) => '╓',
            (Thick, Solid) | (Thick, Dashed) => '┍',
            (Thick, Thick) | (Thick, DashedThick) => '┏',
            (Double, Solid) | (Double, Dashed) => '╒',
            (Double, Double) => '╔',
            (Tight, Tight) => '▗',
            (Block, Block) => '█',
            (Block, _) => '▐',
            (_, Block) => '▄',
            (HalfInner, _) => '▗',
            (HalfOuter, HalfOuter) => '▛',
            (HalfOuter, _) => '█',
            _ => ' ',
        }
    }

    /// Get character at top-right corner
    pub fn top_right(self, right: Self) -> char {
        use Line::*;
        match (self, right) {
            (Solid, Solid) | (Solid, Dashed) => '╮',
            (Solid, Thick) | (Solid, DashedThick) => '┒',
            (Solid, Double) => '╖',
            (Thick, Solid) | (Thick, Dashed) => '┑',
            (Thick, Thick) | (Thick, DashedThick) => '┓',
            (Double, Solid) | (Double, Dashed) => '╕',
            (Double, Double) => '╗',
            (Tight, Tight) => '▖',
            (HalfInner, HalfInner) => '▖',
            (Block, Block) => '█',
            (Block, _) => '▌',
            (_, Block) => '▄',
            (HalfOuter, HalfOuter) => '▜',
            _ => ' ',
        }
    }

    /// Get character at bottom edge
    pub fn bottom(self) -> char {
        use Line::*;
        match self {
            Solid => '─',
            Thick => '━',
            Double => '═',
            Dashed => '╌',
            DashedThick => '╍',
            Tight => '▔',
            HalfInner => '▀',
            Block => '█',
            HalfOuter => '▄',
        }
    }

    /// Get character at right edge
    pub fn right(self) -> char {
        use Line::*;
        match self {
            Solid => '│',
            Thick => '┃',
            Double => '║',
            Dashed => '┆',
            DashedThick => '┇',
            Tight => '▏',
            Block => '█',
            HalfInner => '▌',
            HalfOuter => '▐',
        }
    }

    /// Get character at bottom-left corner
    pub fn bottom_left(self, left: Self) -> char {
        use Line::*;
        match (self, left) {
            (Solid, Solid) | (Solid, Dashed) => '╰',
            (Solid, Thick) | (Solid, DashedThick) => '┖',
            (Solid, Double) => '╙',
            (Thick, Solid) | (Thick, Dashed) => '┕',
            (Thick, Thick) | (Thick, DashedThick) => '┗',
            (Double, Solid) | (Double, Dashed) => '╘',
            (Double, Double) => '╚',
            (Tight, Tight) => '▝',
            (HalfInner, HalfInner) => '▝',
            (Block, Block) => '█',
            (Block, _) => '▐',
            (_, Block) => '▀',
            (HalfOuter, HalfOuter) => '▙',
            _ => ' ',
        }
    }

    /// Get character at bottom-right corner
    pub fn bottom_right(self, right: Self) -> char {
        use Line::*;
        match (self, right) {
            (Solid, Solid) | (Solid, Dashed) => '╯',
            (Solid, Thick) | (Solid, DashedThick) => '┚',
            (Solid, Double) => '╜',
            (Thick, Solid) | (Thick, Dashed) => '┙',
            (Thick, Thick) | (Thick, DashedThick) => '┛',
            (Double, Solid) | (Double, Dashed) => '╛',
            (Double, Double) => '╝',
            (Tight, Tight) => '▘',
            (HalfInner, HalfInner) => '▘',
            (Block, Block) => '█',
            (Block, _) => '▌',
            (_, Block) => '▀',
            (HalfOuter, HalfOuter) => '▟',
            _ => ' ',
        }
    }
}
