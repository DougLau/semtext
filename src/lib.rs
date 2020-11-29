// lib.rs      semtext crate.
//
// Copyright (c) 2020  Douglas Lau
//

mod area;
mod error;
mod screen;

pub use crate::area::{Area, Dim, Edge, Pos};
pub use crate::error::{Error, Result};
pub use crate::screen::{Glyph, Screen};
