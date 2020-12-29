// event.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{Dim, Pos};
use crate::Result;
use crossterm::event::Event as CtEvent;
use crossterm::event::MouseButton as CtMouseButton;
use crossterm::event::MouseEvent as CtMouseEvent;
use crossterm::event::{KeyCode, KeyModifiers, MouseEventKind};

/// Navigation keys
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum NavKey {
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
pub enum FunKey {
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
    Navigation(NavKey),
    Function(FunKey),
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
            Esc => KeyPress::Navigation(NavKey::Esc),
            Enter => KeyPress::Navigation(NavKey::Enter),
            Backspace => KeyPress::Navigation(NavKey::Backspace),
            Delete => KeyPress::Navigation(NavKey::Delete),
            Insert => KeyPress::Navigation(NavKey::Insert),
            Tab => KeyPress::Navigation(NavKey::Tab),
            BackTab => KeyPress::Navigation(NavKey::BackTab),
            Left => KeyPress::Navigation(NavKey::Left),
            Right => KeyPress::Navigation(NavKey::Right),
            Up => KeyPress::Navigation(NavKey::Up),
            Down => KeyPress::Navigation(NavKey::Down),
            Home => KeyPress::Navigation(NavKey::Home),
            End => KeyPress::Navigation(NavKey::End),
            PageUp => KeyPress::Navigation(NavKey::PageUp),
            PageDown => KeyPress::Navigation(NavKey::PageDown),
            F(1) => KeyPress::Function(FunKey::F1),
            F(2) => KeyPress::Function(FunKey::F2),
            F(3) => KeyPress::Function(FunKey::F3),
            F(4) => KeyPress::Function(FunKey::F4),
            F(5) => KeyPress::Function(FunKey::F5),
            F(6) => KeyPress::Function(FunKey::F6),
            F(7) => KeyPress::Function(FunKey::F7),
            F(8) => KeyPress::Function(FunKey::F8),
            F(9) => KeyPress::Function(FunKey::F9),
            F(10) => KeyPress::Function(FunKey::F10),
            F(11) => KeyPress::Function(FunKey::F11),
            F(12) => KeyPress::Function(FunKey::F12),
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
        use MouseEventKind::*;
        match ev.kind {
            Down(btn) => Self::ButtonDown(btn.into()),
            Up(btn) => Self::ButtonUp(btn.into()),
            Drag(btn) => Self::Drag(Some(btn.into())),
            Moved => Self::Drag(None),
            ScrollDown => Self::ScrollDown(),
            ScrollUp => Self::ScrollUp(),
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

impl From<CtMouseEvent> for Pos {
    fn from(ev: CtMouseEvent) -> Self {
        Pos::new(ev.column, ev.row)
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
                ModKeys::from(mev.modifiers),
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
