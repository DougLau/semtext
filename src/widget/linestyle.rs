// linestyle.rs
//
// Copyright (c) 2020  Douglas P Lau
//

/// Styles for borders
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineStyle {
    /// Solid line (thicker with accent)
    ///
    /// ```text
    ///   ╭─────╮           ┍━━━━━┓
    ///   │     │           │     ┃
    ///   │     │           │     ┃
    ///   ╰─────╯           ╰─────┚
    /// Edge::NONE      Edge::TOP_RIGHT
    ///                     Accent
    /// ```
    Solid,
    /// Solid line (doubled with accent)
    ///
    /// ```text
    ///   ╭─────╮           ╓────╮
    ///   │     │           ║    │
    ///   │     │           ║    │
    ///   ╰─────╯           ╚════╛
    /// Edge::NONE     Edge::BOTTOM_LEFT
    ///                     Accent
    /// ```
    Double,
    /// Tightly packed line (thicker with accent)
    ///
    /// ```text
    ///    ▁▁▁▁             ▗▄▄▄▄
    ///   ▕    ▏            ▐    ▏
    ///   ▕    ▏            ▐    ▏
    ///    ▔▔▔▔              ▔▔▔▔
    /// Edge::NONE      Edge::TOP_LEFT
    ///                     Accent
    /// ```
    Tight,
    /// Dashed line (thicker with accent)
    ///
    /// ```text
    ///   ╭╌╌╌╌╮            ┏╍╍╍╍┑
    ///   ┆    ┆            ┇    ┆
    ///   ┆    ┆            ┇    ┆
    ///   ╰╌╌╌╌╯            ┖╌╌╌╌╯
    /// Edge::NONE      Edge::TOP_LEFT
    ///                     Accent
    /// ```
    Dashed,
    /// Block line (full with accent)
    ///
    /// ```text
    ///   ▗▄▄▄▄▖            ▗▄▄▄▄▄
    ///   ▐    ▌            ▐    █
    ///   ▐    ▌            ▐    █
    ///   ▝▀▀▀▀▘            ▐█████
    /// Edge::NONE      Edge::BOTTOM_RIGHT
    ///                     Accent
    /// ```
    Block,
    /// Outer block line (full with accent)
    ///
    /// ```text
    ///   ▛▀▀▀▀▜            ▛▀▀▀▀▜
    ///   ▌    ▐            ▌    ▐
    ///   ▌    ▐            ▌    ▐
    ///   ▙▄▄▄▄▟            ██████
    /// Edge::NONE       Edge::BOTTOM
    ///                     Accent
    /// ```
    OuterBlock,
}

impl Default for LineStyle {
    fn default() -> Self {
        LineStyle::Solid
    }
}
