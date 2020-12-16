// lib.rs      semtext crate.
//
// Copyright (c) 2020  Douglas Lau
//
//! Semtext is a Rust library for building text user interfaces, or **TUI**s.
//! It depends on the excellent [crossterm] library, which allows it to work
//! seamlessly on Linux, Mac or Windows.
//!
//! [crossterm]: https://github.com/crossterm-rs/crossterm

#![forbid(unsafe_code)]

mod bbox;
mod bounds;
mod error;
mod layout;
mod screen;
pub mod style;
pub mod widget;

pub use crate::bbox::BBox;
use crate::bbox::Dim;
pub use crate::bounds::AreaBound;
use crate::bounds::LengthBound;
pub use crate::error::{Error, Result};
pub use crate::layout::{GridItem, Layout};
pub use crate::screen::{Cells, Glyph, IntoGlyph, Screen};
pub use crate::widget::Widget;
