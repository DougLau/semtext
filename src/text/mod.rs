// text/mod.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//! Text styles and themes

mod color;
mod glyph;
mod outline;
mod style;
mod theme;

pub use color::{Color, Intensity};
pub use glyph::{Glyph, IntoGlyph};
pub use outline::{Corner, Outline, Stroke};
pub use style::{Appearance, Style, Weight};
pub use theme::Theme;
