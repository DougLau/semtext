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

mod action;
mod error;
pub mod layout;
mod screen;
pub mod style;
mod traits;
pub mod widget;

pub use crate::action::{Action, KeyMap};
pub use crate::error::Error;
pub(crate) use crate::error::Result;
pub use crate::screen::Screen;
pub use crate::traits::Widget;
