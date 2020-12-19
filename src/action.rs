// action.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Action
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Quit(),
}

/// Event action mapping
pub struct EventActions {
    actions: HashMap<Event, Action>,
}

impl Default for EventActions {
    fn default() -> Self {
        let mut actions = HashMap::new();
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        actions.insert(Event::Key(key), Action::Quit());
        Self { actions }
    }
}

impl EventActions {
    pub fn lookup(&self, event: &Event) -> Option<Action> {
        self.actions.get(event).cloned()
    }
}
