// outline.rs
//
// Copyright (c) 2020  Douglas P Lau
//

/// Styles for border outlines
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Outline {
    /// Empty outline (all spaces)
    Empty,
    /// Solid outline
    ///
    /// ```text
    ///   ╭───╮
    ///   │   │
    ///   ╰───╯
    /// ```
    Solid,
    /// Thick solid outline
    ///
    /// ```text
    ///   ┏━━━┓
    ///   ┃   ┃
    ///   ┗━━━┛
    /// ```
    Thick,
    /// Doubled solid outline
    ///
    /// ```text
    ///   ╔═══╗
    ///   ║   ║
    ///   ╚═══╝
    /// ```
    Double,
    /// Dashed outline
    ///
    /// ```text
    ///   ╭╌╌╌╮
    ///   ┆   ┆
    ///   ╰╌╌╌╯
    /// ```
    Dashed,
    /// Thick dashed outline
    ///
    /// ```text
    ///   ┏╍╍╍┓
    ///   ┇   ┇
    ///   ┗╍╍╍┛
    /// ```
    DashedThick,
    /// Tightly packed outline
    ///
    /// ```text
    ///    ▁▁▁
    ///   ▕   ▏
    ///    ▔▔▔
    /// ```
    Tight,
    /// Half block outline
    ///
    /// ```text
    ///   ▗▄▄▄▖
    ///   ▐   ▌
    ///   ▝▀▀▀▘
    /// ```
    HalfInner,
    /// Outer block outline
    ///
    /// ```text
    ///   ▛▀▀▀▜
    ///   ▌   ▐
    ///   ▙▄▄▄▟
    /// ```
    HalfOuter,
    /// Full Block outline
    ///
    /// ```text
    ///   █████
    ///   █   █
    ///   █████
    /// ```
    Block,
    // Drop Shadow outline (legacy symbols?)
    //
    // ```text
    //    ▒▒▒◣
    //    ◥███
    // ```
    // Shadow,
    // Meduim Shade Drop Shadow outline (legacy symbols)
    //
    // ```text
    //    ███🮟
    //    🮝▒▒▒
    // ```
    // MediumShadow,
}

impl Default for Outline {
    fn default() -> Self {
        Outline::Solid
    }
}

impl Outline {
    /// Get character at top edge
    pub fn top(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Solid => '─',
            Thick => '━',
            Double => '═',
            Dashed => '╌',
            DashedThick => '╍',
            Tight => '▁',
            HalfInner => '▄',
            HalfOuter => '▀',
            Block => '█',
        }
    }

    /// Get character at left edge
    pub fn left(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Solid => '│',
            Thick => '┃',
            Double => '║',
            Dashed => '┆',
            DashedThick => '┇',
            Tight => '▕',
            HalfInner => '▐',
            HalfOuter => '▌',
            Block => '█',
        }
    }

    /// Get character at bottom edge
    pub fn bottom(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Solid => '─',
            Thick => '━',
            Double => '═',
            Dashed => '╌',
            DashedThick => '╍',
            Tight => '▔',
            HalfInner => '▀',
            HalfOuter => '▄',
            Block => '█',
        }
    }

    /// Get character at right edge
    pub fn right(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Solid => '│',
            Thick => '┃',
            Double => '║',
            Dashed => '┆',
            DashedThick => '┇',
            Tight => '▏',
            HalfInner => '▌',
            HalfOuter => '▐',
            Block => '█',
        }
    }

    /// Get character at top-left corner
    pub fn top_left(self, left: Self) -> char {
        use Outline::*;
        match (self, left) {
            (Solid, Solid) | (Solid, Dashed) => '╭',
            (Solid, Thick) | (Solid, DashedThick) => '┎',
            (Solid, Double) => '╓',
            (Solid, Tight) => '╶',
            (Thick, Solid) | (Thick, Dashed) => '┍',
            (Thick, Thick) | (Thick, DashedThick) => '┏',
            (Thick, Double) | (Thick, Tight) => '╺',
            (Double, Solid) | (Double, Dashed) => '╒',
            (Double, Double) => '╔',
            (Dashed, Solid) | (Dashed, Dashed) => '╭',
            (Dashed, Thick) | (Dashed, DashedThick) => '┎',
            (Dashed, Double) => '╓',
            (Dashed, Tight) => '╶',
            (DashedThick, Solid) | (DashedThick, Dashed) => '┍',
            (DashedThick, Thick) | (DashedThick, DashedThick) => '┏',
            (DashedThick, Double) | (DashedThick, Tight) => '╺',
            (Tight, Tight) => ' ',
            (Tight, _) => '▁',
            (HalfInner, _) => '▗',
            (HalfOuter, HalfInner) => '▐',
            (HalfOuter, HalfOuter) => '▛',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▝',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▐',
            (_, Thick) | (_, DashedThick) => '╻',
            (_, Block) => '▄',
            (_, HalfInner) => '▗',
            (_, HalfOuter) => '▖',
            _ => left.left(),
        }
    }

    /// Get character at top-right corner
    pub fn top_right(self, right: Self) -> char {
        use Outline::*;
        match (self, right) {
            (Solid, Solid) | (Solid, Dashed) => '╮',
            (Solid, Thick) | (Solid, DashedThick) => '┒',
            (Solid, Double) => '╖',
            (Solid, Tight) => '╴',
            (Thick, Solid) | (Thick, Dashed) => '┑',
            (Thick, Thick) | (Thick, DashedThick) => '┓',
            (Thick, Double) | (Thick, Tight) => '╸',
            (Double, Solid) | (Double, Dashed) => '╕',
            (Double, Double) => '╗',
            (Dashed, Solid) | (Dashed, Dashed) => '╮',
            (Dashed, Thick) | (Dashed, DashedThick) => '┒',
            (Dashed, Double) => '╖',
            (Dashed, Tight) => '╴',
            (DashedThick, Solid) | (DashedThick, Dashed) => '┑',
            (DashedThick, Thick) | (DashedThick, DashedThick) => '┓',
            (DashedThick, Double) | (DashedThick, Tight) => '╸',
            (Tight, Tight) => ' ',
            (Tight, _) => '▁',
            (HalfInner, _) => '▖',
            (HalfOuter, HalfInner) => '▌',
            (HalfOuter, HalfOuter) => '▜',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▘',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▌',
            (_, Thick) | (_, DashedThick) => '╻',
            (_, Block) => '▄',
            (_, HalfInner) => '▖',
            (_, HalfOuter) => '▗',
            _ => right.right(),
        }
    }

    /// Get character at bottom-left corner
    pub fn bottom_left(self, left: Self) -> char {
        use Outline::*;
        match (self, left) {
            (Solid, Solid) | (Solid, Dashed) => '╰',
            (Solid, Thick) | (Solid, DashedThick) => '┖',
            (Solid, Double) => '╙',
            (Solid, Tight) => '╶',
            (Thick, Solid) | (Thick, Dashed) => '┕',
            (Thick, Thick) | (Thick, DashedThick) => '┗',
            (Thick, Double) | (Thick, Tight) => '╺',
            (Double, Solid) | (Double, Dashed) => '╘',
            (Double, Double) => '╚',
            (Dashed, Solid) | (Dashed, Dashed) => '╰',
            (Dashed, Thick) | (Dashed, DashedThick) => '┖',
            (Dashed, Double) => '╙',
            (Dashed, Tight) => '╶',
            (DashedThick, Solid) | (DashedThick, Dashed) => '┕',
            (DashedThick, Thick) | (DashedThick, DashedThick) => '┗',
            (DashedThick, Double) | (DashedThick, Tight) => '╺',
            (Tight, Tight) => ' ',
            (Tight, _) => '▔',
            (HalfInner, _) => '▝',
            (HalfOuter, HalfInner) => '▐',
            (HalfOuter, HalfOuter) => '▙',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▗',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▐',
            (_, Thick) | (_, DashedThick) => '╹',
            (_, Block) => '▀',
            (_, HalfInner) => '▝',
            (_, HalfOuter) => '▘',
            _ => left.left(),
        }
    }

    /// Get character at bottom-right corner
    pub fn bottom_right(self, right: Self) -> char {
        use Outline::*;
        match (self, right) {
            (Solid, Solid) | (Solid, Dashed) => '╯',
            (Solid, Thick) | (Solid, DashedThick) => '┚',
            (Solid, Double) => '╜',
            (Solid, Tight) => '╴',
            (Thick, Solid) | (Thick, Dashed) => '┙',
            (Thick, Thick) | (Thick, DashedThick) => '┛',
            (Thick, Double) | (Thick, Tight) => '╸',
            (Double, Solid) | (Double, Dashed) => '╛',
            (Double, Double) => '╝',
            (Dashed, Solid) | (Dashed, Dashed) => '╯',
            (Dashed, Thick) | (Dashed, DashedThick) => '┚',
            (Dashed, Double) => '╜',
            (Dashed, Tight) => '╴',
            (DashedThick, Solid) | (DashedThick, Dashed) => '┙',
            (DashedThick, Thick) | (DashedThick, DashedThick) => '┛',
            (DashedThick, Double) | (DashedThick, Tight) => '╸',
            (Tight, Tight) => ' ',
            (Tight, _) => '▔',
            (HalfInner, _) => '▘',
            (HalfOuter, HalfInner) => '▌',
            (HalfOuter, HalfOuter) => '▟',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▖',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▌',
            (_, Thick) | (_, DashedThick) => '╹',
            (_, Block) => '▀',
            (_, HalfInner) => '▘',
            (_, HalfOuter) => '▝',
            _ => right.right(),
        }
    }
}
