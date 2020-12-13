// theme.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crossterm::style::Color;

/// Style theme
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Foreground text color
    foreground: Color,
    /// Background color
    background: Color,
    /// Primary widget color
    primary: Color,
    /// Secondary widget color
    secondary: Color,
    /// Tertiary widget color
    tertiary: Color,
}

impl Default for Theme {
    fn default() -> Self {
        // These are terminal base colors
        // They don't necessarily match the names
        let foreground = Color::White;
        let background = Color::Black;
        let primary = Color::Yellow;
        let secondary = Color::Cyan;
        let tertiary = Color::Magenta;
        Self {
            foreground,
            background,
            primary,
            secondary,
            tertiary,
        }
    }
}

impl Theme {
    /// Set the foreground color
    pub fn with_foreground(mut self, clr: Color) -> Self {
        self.foreground = clr;
        self
    }

    pub(crate) fn foreground(&self) -> Color {
        self.foreground
    }

    pub(crate) fn background(&self) -> Color {
        self.background
    }

    pub(crate) fn primary(&self) -> Color {
        self.primary
    }
}
