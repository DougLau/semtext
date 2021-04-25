// input/mod.rs
//
// Copyright (c) 2020-2021  Douglas P Lau
//
//! Keyboard and mouse input handling

mod action;
mod event;

pub use action::{Action, KeyMap};
pub(crate) use event::Event;
pub use event::{
    FocusEvent, FunKey, KeyPress, ModKeys, MouseButton, MouseEvent, NavKey,
};
