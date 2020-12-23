// input/mod.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//! Input stuff

mod action;
mod event;

pub use action::{Action, KeyMap};
pub use event::{
    Event, FunKey, KeyPress, ModKeys, MouseButton, MouseEvent, NavKey,
};
