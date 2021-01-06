// lib.rs      semtext crate.
//
// Copyright (c) 2020  Douglas Lau
//
//! Semtext is a Rust library for building text user interfaces, or **TUI**s.
//!
//! ## Example
//! ```no_run
//! use semtext::{grid_area, input::Action, widget::Label, Screen, Widget};
//!
//! async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut screen = Screen::new()?;
//!     let a = Label::new("Hello!").into_button();
//!     let grid = grid_area!(
//!         [. . .]
//!         [. a .]
//!         [. . .]
//!     )?;
//!     while screen.step(&grid).await? != Action::Quit() {}
//!     Ok(())
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     futures::executor::block_on(async_main())
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
