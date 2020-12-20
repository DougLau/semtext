// style.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::text::{Color, Intensity};
use crossterm::style::{Attribute, Attributes};

/// Text Appearance
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Appearance {
    /// Crossterm text attributes
    attributes: Attributes,
}

/// Text Style
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    /// Background color
    background: Color,
    /// Foreground text color
    foreground: Color,
    /// Text appearance
    appearance: Appearance,
}

impl From<Appearance> for Attributes {
    fn from(app: Appearance) -> Self {
        app.attributes
    }
}

impl Appearance {
    /// Set `italic` text appearance
    pub fn with_italic(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::Italic);
        } else {
            self.attributes.unset(Attribute::Italic);
        }
        self
    }

    /// Set `bold` text appearance
    pub fn with_bold(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::Bold);
        } else {
            self.attributes.unset(Attribute::Bold);
        }
        self
    }

    /// Set `strikethrough` text appearance
    pub fn with_strikethrough(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::CrossedOut);
        } else {
            self.attributes.unset(Attribute::CrossedOut);
        }
        self
    }

    /// Set `underline` text appearance
    pub fn with_underline(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::Underlined);
        } else {
            self.attributes.unset(Attribute::Underlined);
        }
        self
    }

    /// Set `reverse` text appearance
    pub fn with_reverse(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::Reverse);
        } else {
            self.attributes.unset(Attribute::Reverse);
        }
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        let background = Color::Black(Intensity::Normal);
        let foreground = Color::White(Intensity::Bright);
        let appearance = Appearance::default();
        Self { background, foreground, appearance }
    }
}

impl Style {
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

    /// Set the text appearance
    pub fn with_appearance(mut self, app: Appearance) -> Self {
        self.appearance = app;
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

    /// Get the text appearance
    pub fn appearance(&self) -> Appearance {
        self.appearance
    }
}
