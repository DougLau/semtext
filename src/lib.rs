// lib.rs      semtext crate.
//
// Copyright (c) 2020  Douglas Lau
//
//! Semtext is a Rust library for building text user interfaces, or **TUI**s.
//!
//! ## Example
//! ```no_run
//! use semtext::{grid_area, input::Action, widget::Button, Screen};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut screen = Screen::new()?;
//!     let a = Button::new("Hello!");
//!     let grid = grid_area!(
//!         [. . .]
//!         [. a .]
//!         [. . .]
//!     )?;
//!     while screen.step(&grid)? != Action::Quit() {}
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]

mod error;
pub mod input;
pub mod layout;
mod screen;
pub mod text;
mod traits;
pub mod widget;

pub use crate::error::Error;
pub(crate) use crate::error::Result;
pub use crate::screen::Screen;
pub use crate::traits::Widget;
