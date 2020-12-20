// action.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Action
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// Quit application
    Quit(),
}

/// Key action mapping
///
/// FIXME: don't expose crossterm KeyEvent
pub struct KeyMap {
    /// Mapping of key events to actions
    map: HashMap<KeyEvent, Action>,
}

impl Default for KeyMap {
    fn default() -> Self {
        let mut map = HashMap::new();
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        map.insert(key, Action::Quit());
        Self { map }
    }
}

impl KeyMap {
    /// Lookup an [Action] from a key event
    pub fn lookup(&self, event: &KeyEvent) -> Option<Action> {
        self.map.get(event).cloned()
    }
}
