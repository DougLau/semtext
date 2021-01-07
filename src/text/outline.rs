// outline.rs
//
// Copyright (c) 2020  Douglas P Lau
//

/// Outline corner style
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Corner {
    /// Square corners
    Square,
    /// Rounded corners
    Rounded,
}

/// Outline stroke style
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stroke {
    /// Solid outline
    Solid,
    /// Dashed outline
    Dashed,
}

/// Outline style
///
/// Outlines require font support for one of more **Unicode Blocks**:
/// - **Basic Latin** (U+0000 - U+007F)
/// - **Box Drawing** (U+2500 - U+257F)
/// - **Block Elements** (U+2580 - U+259F)
/// - **Geometric Shapes** (U+25A0 - U+25FF)
/// - **Symbols For Legacy Computing** (U+1FB00 - U+1FBFF)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outline {
    /// Empty outline (all spaces)
    ///
    /// Required: **Basic Latin**
    Empty,
    /// Light outline
    ///
    /// ```text
    ///         Solid Dashed
    ///         ┌───┐ ┌╌╌╌┐
    /// Square  │   │ ┆   ┆
    ///         └───┘ └╌╌╌┘
    ///         ╭───╮ ╭╌╌╌╮
    /// Rounded │   │ ┆   ┆
    ///         ╰───╯ ╰╌╌╌╯
    /// ```
    ///
    /// Required: **Box Drawing**
    Light(Stroke, Corner),
    /// Heavy outline
    ///
    /// ```text
    ///   Solid Dashed
    ///   ┏━━━┓ ┏╍╍╍┓
    ///   ┃   ┃ ┇   ┇
    ///   ┗━━━┛ ┗╍╍╍┛
    /// ```
    /// Required: **Box Drawing**
    Heavy(Stroke),
    /// Doubled solid outline
    ///
    /// ```text
    ///   ╔═══╗
    ///   ║   ║
    ///   ╚═══╝
    /// ```
    /// Required: **Box Drawing**
    Double,
    /// Tightly packed outline
    ///
    /// ```text
    ///    ▁▁▁
    ///   ▕   ▏
    ///    ▔▔▔
    /// ```
    /// Required: **Box Drawing**
    Tight,
    /// Half block outline
    ///
    /// ```text
    ///   ▗▄▄▄▖
    ///   ▐   ▌
    ///   ▝▀▀▀▘
    /// ```
    /// Required: **Block Elements**
    HalfInner,
    /// Outer block outline
    ///
    /// ```text
    ///   ▛▀▀▀▜
    ///   ▌   ▐
    ///   ▙▄▄▄▟
    /// ```
    /// Required: **Block Elements**
    HalfOuter,
    /// Full Block outline
    ///
    /// ```text
    ///   █████
    ///   █   █
    ///   █████
    /// ```
    /// Required: **Block Elements**
    Block,
    /// Medium Shade outline
    /// ```text
    ///    ▒▒▒▒▒
    ///    ▒   ▒
    ///    ▒▒▒▒▒
    /// ```
    /// Required: **Block Elements**
    MediumShade,
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
        Outline::Light(Stroke::Solid, Corner::Square)
    }
}

impl Outline {
    /// Get character at top edge
    pub fn top(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Light(Stroke::Solid, _) => '─',
            Light(Stroke::Dashed, _) => '╌',
            Heavy(Stroke::Solid) => '━',
            Heavy(Stroke::Dashed) => '╍',
            Double => '═',
            Tight => '▁',
            HalfInner => '▄',
            HalfOuter => '▀',
            Block => '█',
            MediumShade => '▒',
        }
    }

    /// Get character at left edge
    pub fn left(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Light(Stroke::Solid, _) => '│',
            Light(Stroke::Dashed, _) => '┆',
            Heavy(Stroke::Solid) => '┃',
            Heavy(Stroke::Dashed) => '┇',
            Double => '║',
            Tight => '▕',
            HalfInner => '▐',
            HalfOuter => '▌',
            Block => '█',
            MediumShade => '▒',
        }
    }

    /// Get character at bottom edge
    pub fn bottom(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Light(Stroke::Solid, _) => '─',
            Light(Stroke::Dashed, _) => '╌',
            Heavy(Stroke::Solid) => '━',
            Heavy(Stroke::Dashed) => '╍',
            Double => '═',
            Tight => '▔',
            HalfInner => '▀',
            HalfOuter => '▄',
            Block => '█',
            MediumShade => '▒',
        }
    }

    /// Get character at right edge
    pub fn right(self) -> char {
        use Outline::*;
        match self {
            Empty => ' ',
            Light(Stroke::Solid, _) => '│',
            Light(Stroke::Dashed, _) => '┆',
            Heavy(Stroke::Solid) => '┃',
            Heavy(Stroke::Dashed) => '┇',
            Double => '║',
            Tight => '▏',
            HalfInner => '▌',
            HalfOuter => '▐',
            Block => '█',
            MediumShade => '▒',
        }
    }

    /// Get character at top-left corner
    pub fn top_left(self, left: Self) -> char {
        use Corner::*;
        use Outline::*;
        match (self, left) {
            (Light(_, Square), Light(_, Square)) => '┌',
            (Light(_, _), Light(_, _)) => '╭',
            (Light(_, _), Heavy(_)) => '┎',
            (Light(_, _), Double) => '╓',
            (Light(_, _), Tight) => '╶',
            (Heavy(_), Light(_, _)) => '┍',
            (Heavy(_), Heavy(_)) => '┏',
            (Heavy(_), Double) | (Heavy(_), Tight) => '╺',
            (Double, Light(_, _)) => '╒',
            (Double, Double) => '╔',
            (Tight, Tight) => ' ',
            (Tight, _) => '▁',
            (HalfInner, _) => '▗',
            (HalfOuter, HalfInner) => '▐',
            (HalfOuter, HalfOuter) => '▛',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▝',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▐',
            (_, Heavy(_)) => '╻',
            (_, Block) => '▄',
            (_, HalfInner) => '▗',
            (_, HalfOuter) => '▖',
            _ => left.left(),
        }
    }

    /// Get character at top-right corner
    pub fn top_right(self, right: Self) -> char {
        use Corner::*;
        use Outline::*;
        match (self, right) {
            (Light(_, Square), Light(_, Square)) => '┐',
            (Light(_, _), Light(_, _)) => '╮',
            (Light(_, _), Heavy(_)) => '┒',
            (Light(_, _), Double) => '╖',
            (Light(_, _), Tight) => '╴',
            (Heavy(_), Light(_, _)) => '┑',
            (Heavy(_), Heavy(_)) => '┓',
            (Heavy(_), Double) | (Heavy(_), Tight) => '╸',
            (Double, Light(_, _)) => '╕',
            (Double, Double) => '╗',
            (Tight, Tight) => ' ',
            (Tight, _) => '▁',
            (HalfInner, _) => '▖',
            (HalfOuter, HalfInner) => '▌',
            (HalfOuter, HalfOuter) => '▜',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▘',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▌',
            (_, Heavy(_)) => '╻',
            (_, Block) => '▄',
            (_, HalfInner) => '▖',
            (_, HalfOuter) => '▗',
            _ => right.right(),
        }
    }

    /// Get character at bottom-left corner
    pub fn bottom_left(self, left: Self) -> char {
        use Corner::*;
        use Outline::*;
        match (self, left) {
            (Light(_, Square), Light(_, Square)) => '└',
            (Light(_, _), Light(_, _)) => '╰',
            (Light(_, _), Heavy(_)) => '┖',
            (Light(_, _), Double) => '╙',
            (Light(_, _), Tight) => '╶',
            (Heavy(_), Light(_, _)) => '┕',
            (Heavy(_), Heavy(_)) => '┗',
            (Heavy(_), Double) | (Heavy(_), Tight) => '╺',
            (Double, Light(_, _)) => '╘',
            (Double, Double) => '╚',
            (Tight, Tight) => ' ',
            (Tight, _) => '▔',
            (HalfInner, _) => '▝',
            (HalfOuter, HalfInner) => '▐',
            (HalfOuter, HalfOuter) => '▙',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▗',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▐',
            (_, Heavy(_)) => '╹',
            (_, Block) => '▀',
            (_, HalfInner) => '▝',
            (_, HalfOuter) => '▘',
            _ => left.left(),
        }
    }

    /// Get character at bottom-right corner
    pub fn bottom_right(self, right: Self) -> char {
        use Corner::*;
        use Outline::*;
        match (self, right) {
            (Light(_, Square), Light(_, Square)) => '┘',
            (Light(_, _), Light(_, _)) => '╯',
            (Light(_, _), Heavy(_)) => '┚',
            (Light(_, _), Double) => '╜',
            (Light(_, _), Tight) => '╴',
            (Heavy(_), Light(_, _)) => '┙',
            (Heavy(_), Heavy(_)) => '┛',
            (Heavy(_), Double) | (Heavy(_), Tight) => '╸',
            (Double, Light(_, _)) => '╛',
            (Double, Double) => '╝',
            (Tight, Tight) => ' ',
            (Tight, _) => '▔',
            (HalfInner, _) => '▘',
            (HalfOuter, HalfInner) => '▌',
            (HalfOuter, HalfOuter) => '▟',
            (HalfOuter, Block) => '█',
            (HalfOuter, _) => '▖',
            (Block, Block) | (Block, HalfOuter) => '█',
            (Block, _) => '▌',
            (_, Heavy(_)) => '╹',
            (_, Block) => '▀',
            (_, HalfInner) => '▘',
            (_, HalfOuter) => '▝',
            _ => right.right(),
        }
    }
}
