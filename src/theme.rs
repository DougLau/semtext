// theme.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crossterm::style::Color as Clr;

/// ANSI color intensity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intensity {
    /// Normal (dark) color intensity
    Normal,
    /// Bright color intensity
    Bright,
}

/// Text Colors
///
/// Colors can be specified using one of the standard 16 ANSI colors, or as
/// `Rgb` 24-bit *true color*.  In most cases, it is best to use the ANSI
/// colors, since it allows the user to define their own preferences for all
/// their terminal apps.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    /// ANSI color 0 *black*, and 8 *dark gray* (bright)
    Black(Intensity),
    /// ANSI color 1 *red*, and 9 (bright)
    Red(Intensity),
    /// ANSI color 2 *green*, and 10 (bright)
    Green(Intensity),
    /// ANSI color 3 *yellow*, and 11 (bright)
    Yellow(Intensity),
    /// ANSI color 4 *blue*, and 12 (bright)
    Blue(Intensity),
    /// ANSI color 5 *magenta*, and 13 (bright)
    Magenta(Intensity),
    /// ANSI color 6 *cyan*, and 14 (bright)
    Cyan(Intensity),
    /// ANSI color 7 *light gray*, and 15 *white* (bright)
    White(Intensity),
    /// Red, green, blue *true color*
    Rgb(u8, u8, u8),
}

impl From<Color> for Clr {
    fn from(clr: Color) -> Self {
        use Color::*;
        match clr {
            Black(Intensity::Normal) => Clr::Black,
            Black(Intensity::Bright) => Clr::DarkGrey,
            Red(Intensity::Normal) => Clr::DarkRed,
            Red(Intensity::Bright) => Clr::Red,
            Green(Intensity::Normal) => Clr::DarkGreen,
            Green(Intensity::Bright) => Clr::Green,
            Yellow(Intensity::Normal) => Clr::DarkYellow,
            Yellow(Intensity::Bright) => Clr::Yellow,
            Blue(Intensity::Normal) => Clr::DarkBlue,
            Blue(Intensity::Bright) => Clr::Blue,
            Magenta(Intensity::Normal) => Clr::DarkMagenta,
            Magenta(Intensity::Bright) => Clr::Magenta,
            Cyan(Intensity::Normal) => Clr::DarkCyan,
            Cyan(Intensity::Bright) => Clr::Cyan,
            White(Intensity::Normal) => Clr::Grey,
            White(Intensity::Bright) => Clr::White,
            Rgb(r, g, b) => Clr::Rgb { r, g, b },
        }
    }
}

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
        let foreground = Color::White(Intensity::Bright);
        let background = Color::Black(Intensity::Normal);
        let primary = Color::Yellow(Intensity::Bright);
        let secondary = Color::Cyan(Intensity::Bright);
        let tertiary = Color::Magenta(Intensity::Bright);
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

    /// Set the background color
    pub fn with_background(mut self, clr: Color) -> Self {
        self.background = clr;
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

    /// Get the foreground color
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get the background color
    pub fn background(&self) -> Color {
        self.background
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
