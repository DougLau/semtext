// action.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::input::{KeyPress, ModKeys, NavKey};
use crate::layout::Dim;
use std::collections::HashMap;

/// Action
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// Terminal resized
    Resize(Dim),
    /// Redraw required
    Redraw(),
    /// Quit application
    Quit(),
}

/// Key action mapping
pub struct KeyMap {
    /// Mapping of key presses to actions
    map: HashMap<(KeyPress, ModKeys), Action>,
}

impl Default for KeyMap {
    fn default() -> Self {
        let mut map = HashMap::new();
        let key = (KeyPress::Navigation(NavKey::Esc), ModKeys::Empty);
        map.insert(key, Action::Quit());
        Self { map }
    }
}

impl KeyMap {
    /// Lookup an [Action] from a key event
    pub fn lookup(&self, key: KeyPress, mods: ModKeys) -> Option<Action> {
        self.map.get(&(key, mods)).cloned()
    }
}
