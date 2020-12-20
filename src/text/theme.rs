// theme.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::text::{Color, Intensity};

/// Style theme
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Background color
    background: Color,
    /// Foreground text color
    foreground: Color,
    /// Primary widget color
    primary: Color,
    /// Secondary widget color
    secondary: Color,
    /// Tertiary widget color
    tertiary: Color,
}

impl Default for Theme {
    fn default() -> Self {
        let background = Color::Black(Intensity::Normal);
        let foreground = Color::White(Intensity::Bright);
        let primary = Color::Yellow(Intensity::Bright);
        let secondary = Color::Cyan(Intensity::Bright);
        let tertiary = Color::Magenta(Intensity::Bright);
        Self {
            background,
            foreground,
            primary,
            secondary,
            tertiary,
        }
    }
}

impl Theme {
    /// Set the background color
    pub fn with_background(mut self, clr: Color) -> Self {
        self.background = clr;
        self
    }

    /// Set the foreground color
    pub fn with_foreground(mut self, clr: Color) -> Self {
        self.foreground = clr;
        self
    }

    /// Set the primary color
    pub fn with_primary(mut self, clr: Color) -> Self {
        self.primary = clr;
        self
    }

    /// Set the secondary color
    pub fn with_secondary(mut self, clr: Color) -> Self {
        self.secondary = clr;
        self
    }

    /// Set the tertiary color
    pub fn with_tertiary(mut self, clr: Color) -> Self {
        self.tertiary = clr;
        self
    }

    /// Get the background color
    pub fn background(&self) -> Color {
        self.background
    }

    /// Get the foreground color
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get the primary color
    pub fn primary(&self) -> Color {
        self.primary
    }

    /// Get the secondary color
    pub fn secondary(&self) -> Color {
        self.secondary
    }

    /// Get the tertiary color
    pub fn tertiary(&self) -> Color {
        self.tertiary
    }
}
