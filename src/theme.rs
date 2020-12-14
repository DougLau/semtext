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
    /// Standard ANSI *black* (normal) and *dark gray* (bright)
    Black(Intensity),
    /// Standard ANSI *red*
    Red(Intensity),
    /// Standard ANSI *green*
    Green(Intensity),
    /// Standard ANSI *yellow*
    Yellow(Intensity),
    /// Standard ANSI *blue*
    Blue(Intensity),
    /// Standard ANSI *magenta*
    Magenta(Intensity),
    /// Standard ANSI *cyan*
    Cyan(Intensity),
    /// Standard ANSI *light gray* (normal) and *white* (bright)
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
        // These are terminal base colors
        // They don't necessarily match the names
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
