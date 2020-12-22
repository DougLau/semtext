// event.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{Dim, Pos};
use crate::Result;
use crossterm::event::Event as CtEvent;
use crossterm::event::MouseButton as CtMouseButton;
use crossterm::event::MouseEvent as CtMouseEvent;
use crossterm::event::{KeyCode, KeyModifiers};

/// Navigation keys
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum NavigationKey {
    Esc,
    Enter,
    Backspace,
    Delete,
    Insert,
    Tab,
    BackTab,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

/// Function Keys
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum FunctionKey {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

/// Mouse Buttons
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

/// Mouse Events
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseEvent {
    ButtonDown(MouseButton),
    ButtonUp(MouseButton),
    ScrollDown(),
    ScrollUp(),
    Drag(Option<MouseButton>),
}

/// Modifier Keys
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ModKeys {
    Empty,
    Control,
    Alt,
    ControlAlt,
    Shift,
    ControlShift,
    AltShift,
    ControlAltShift,
}

/// Key press event
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum KeyPress {
    Navigation(NavigationKey),
    Function(FunctionKey),
    Character(char),
}

/// Input event
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Resize(Dim),
    Key(KeyPress, ModKeys),
    Mouse(MouseEvent, ModKeys, Pos),
}

impl From<KeyCode> for KeyPress {
    fn from(key: KeyCode) -> Self {
        use KeyCode::*;
        match key {
            Esc => KeyPress::Navigation(NavigationKey::Esc),
            Enter => KeyPress::Navigation(NavigationKey::Enter),
            Backspace => KeyPress::Navigation(NavigationKey::Backspace),
            Delete => KeyPress::Navigation(NavigationKey::Delete),
            Insert => KeyPress::Navigation(NavigationKey::Insert),
            Tab => KeyPress::Navigation(NavigationKey::Tab),
            BackTab => KeyPress::Navigation(NavigationKey::BackTab),
            Left => KeyPress::Navigation(NavigationKey::Left),
            Right => KeyPress::Navigation(NavigationKey::Right),
            Up => KeyPress::Navigation(NavigationKey::Up),
            Down => KeyPress::Navigation(NavigationKey::Down),
            Home => KeyPress::Navigation(NavigationKey::Home),
            End => KeyPress::Navigation(NavigationKey::End),
            PageUp => KeyPress::Navigation(NavigationKey::PageUp),
            PageDown => KeyPress::Navigation(NavigationKey::PageDown),
            F(1) => KeyPress::Function(FunctionKey::F1),
            F(2) => KeyPress::Function(FunctionKey::F2),
            F(3) => KeyPress::Function(FunctionKey::F3),
            F(4) => KeyPress::Function(FunctionKey::F4),
            F(5) => KeyPress::Function(FunctionKey::F5),
            F(6) => KeyPress::Function(FunctionKey::F6),
            F(7) => KeyPress::Function(FunctionKey::F7),
            F(8) => KeyPress::Function(FunctionKey::F8),
            F(9) => KeyPress::Function(FunctionKey::F9),
            F(10) => KeyPress::Function(FunctionKey::F10),
            F(11) => KeyPress::Function(FunctionKey::F11),
            F(12) => KeyPress::Function(FunctionKey::F12),
            Char(c) => KeyPress::Character(c),
            _ => KeyPress::Character('\0'),
        }
    }
}

impl From<CtMouseButton> for MouseButton {
    fn from(btn: CtMouseButton) -> Self {
        match btn {
            CtMouseButton::Left => Self::Left,
            CtMouseButton::Middle => Self::Middle,
            CtMouseButton::Right => Self::Right,
        }
    }
}

impl From<CtMouseEvent> for MouseEvent {
    fn from(ev: CtMouseEvent) -> Self {
        use CtMouseEvent::*;
        match ev {
            Down(btn, _, _, _) => Self::ButtonDown(btn.into()),
            Up(btn, _, _, _) => Self::ButtonUp(btn.into()),
            Drag(btn, _, _, _) => Self::Drag(Some(btn.into())),
            ScrollDown(_, _, _) => Self::ScrollDown(),
            ScrollUp(_, _, _) => Self::ScrollUp(),
        }
    }
}

impl From<KeyModifiers> for ModKeys {
    fn from(mods: KeyModifiers) -> Self {
        // Can't match on bitflags, yuck...
        if mods == KeyModifiers::CONTROL {
            Self::Control
        } else if mods == KeyModifiers::ALT {
            Self::Alt
        } else if mods == KeyModifiers::CONTROL | KeyModifiers::ALT {
            Self::ControlAlt
        } else if mods == KeyModifiers::SHIFT {
            Self::Shift
        } else if mods == KeyModifiers::CONTROL | KeyModifiers::SHIFT {
            Self::ControlShift
        } else if mods == KeyModifiers::ALT | KeyModifiers::SHIFT {
            Self::AltShift
        } else if mods
            == KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT
        {
            Self::ControlAltShift
        } else {
            Self::Empty
        }
    }
}

impl From<CtMouseEvent> for ModKeys {
    fn from(ev: CtMouseEvent) -> Self {
        use CtMouseEvent::*;
        match ev {
            Down(_, _, _, mods) => ModKeys::from(mods),
            Up(_, _, _, mods) => ModKeys::from(mods),
            Drag(_, _, _, mods) => ModKeys::from(mods),
            ScrollDown(_, _, mods) => ModKeys::from(mods),
            ScrollUp(_, _, mods) => ModKeys::from(mods),
        }
    }
}

impl From<CtMouseEvent> for Pos {
    fn from(ev: CtMouseEvent) -> Self {
        use CtMouseEvent::*;
        match ev {
            Down(_, col, row, _) => Pos::new(col, row),
            Up(_, col, row, _) => Pos::new(col, row),
            Drag(_, col, row, _) => Pos::new(col, row),
            ScrollDown(col, row, _) => Pos::new(col, row),
            ScrollUp(col, row, _) => Pos::new(col, row),
        }
    }
}

impl From<CtEvent> for Event {
    fn from(ev: CtEvent) -> Self {
        use CtEvent::*;
        match ev {
            Resize(width, height) => Self::Resize(Dim::new(width, height)),
            Key(kev) => Self::Key(
                KeyPress::from(kev.code),
                ModKeys::from(kev.modifiers),
            ),
            Mouse(mev) => Self::Mouse(
                MouseEvent::from(mev),
                ModKeys::from(mev),
                Pos::from(mev),
            ),
        }
    }
}

impl Event {
    pub(crate) fn read() -> Result<Self> {
        Ok(crossterm::event::read()?.into())
    }
}
