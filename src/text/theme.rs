// theme.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::text::{Color, Intensity, Style};
use crate::widget::{BorderHeight, BorderStyle};

/// Style theme
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Background color
    pub background: Color,
    /// Foreground text color
    pub foreground: Color,
    /// Primary widget color
    pub primary: Color,
    /// Secondary widget color
    pub secondary: Color,
    /// Tertiary widget color
    pub tertiary: Color,
    /// Dark shadow color
    pub dark_shadow: Color,
    /// Light shadow color
    pub light_shadow: Color,
    /// Button border style
    pub button_border: BorderStyle,
}

impl Default for Theme {
    fn default() -> Self {
        let background = Color::Blue(Intensity::Normal);
        let foreground = Color::White(Intensity::Bright);
        let primary = Color::Yellow(Intensity::Bright);
        let secondary = Color::Cyan(Intensity::Bright);
        let tertiary = Color::Yellow(Intensity::Bright);
        let dark_shadow = Color::Black(Intensity::Bright);
        let light_shadow = Color::White(Intensity::Normal);
        let button_border = BorderStyle::Bevel(BorderHeight::Raised);
        Self {
            background,
            foreground,
            primary,
            secondary,
            tertiary,
            dark_shadow,
            light_shadow,
            button_border,
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

    /// Get text style
    pub fn style(&self) -> Style {
        Style::default()
            .with_background(self.background)
            .with_foreground(self.foreground)
    }
}
