// lib.rs      semtext crate.
//
// Copyright (c) 2020  Douglas Lau
//
//! Semtext is a Rust library for building text user interfaces, or **TUI**s.
//! It depends on the excellent [crossterm] library, which supports Linux, Mac
//! and Windows.
//!
//! [crossterm]: https://github.com/crossterm-rs/crossterm

#![forbid(unsafe_code)]

mod error;
mod screen;
pub mod layout;
pub mod style;
pub mod widget;

pub use crate::error::Error;
pub(crate) use crate::error::Result;
pub use crate::screen::{Cells, Screen};
pub use crate::widget::Widget;
