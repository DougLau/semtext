// input/mod.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//! Input stuff

mod action;
mod event;

pub use action::{Action, KeyMap};
pub use event::{
    Event, FunctionKey, KeyPress, ModKeys, MouseButton, MouseEvent,
    NavigationKey,
};
