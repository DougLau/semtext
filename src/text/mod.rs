// text/mod.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//! Text styles and themes

mod glyph;
mod outline;
mod theme;

pub use glyph::{Glyph, IntoGlyph};
pub use outline::Outline;
pub use theme::{Color, Intensity, Theme};
