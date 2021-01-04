// input/mod.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//! Input stuff

mod action;
mod event;

pub use action::{Action, KeyMap};
pub(crate) use event::Event;
pub use event::{
    FocusEvent, FunKey, KeyPress, ModKeys, MouseButton, MouseEvent, NavKey,
};
