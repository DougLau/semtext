// theme.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::text::{Appearance, Color, Intensity, Outline, TextStyle};
use crate::widget::BorderStyle;

/// Widget group
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WidgetGroup {
    /// Normal widget group
    Normal,
    /// Button widget group
    Button,
}

/// Style group
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StyleGroup {
    /// Enabled widget style
    Enabled,
    /// Disabled widget style
    Disabled,
    /// Primary accent style
    Primary,
    /// Hovered style
    Hovered,
    /// Focused style
    Focused,
    /// Interacted style
    Interacted,
    /// Light shadow style
    LightShadow,
    /// Dark shadow style
    DarkShadow,
}

/// Style theme
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Background color
    pub background: Color,
    /// Foreground text color
    pub foreground: Color,
    /// Primary widget color (primary)
    pub primary: Color,
    /// Color for focused elements (secondary)
    pub focused: Color,
    /// Color for interactiing elements (tertiary)
    pub interacting: Color,
    /// Dark shadow color
    pub dark_shadow: Color,
    /// Light shadow color
    pub light_shadow: Color,
    /// Normal border style
    pub normal_border: BorderStyle,
    /// Button border style
    pub button_border: BorderStyle,
}

impl Default for Theme {
    fn default() -> Self {
        let background = Color::Blue(Intensity::Normal);
        let foreground = Color::White(Intensity::Bright);
        let primary = Color::Magenta(Intensity::Bright);
        let focused = Color::Cyan(Intensity::Bright);
        let interacting = Color::Yellow(Intensity::Bright);
        let dark_shadow = Color::Black(Intensity::Bright);
        let light_shadow = Color::White(Intensity::Normal);
        let normal_border = BorderStyle::Simple(Outline::default());
        let button_border = BorderStyle::Bevel(Outline::default());
        Self {
            background,
            foreground,
            primary,
            focused,
            interacting,
            dark_shadow,
            light_shadow,
            normal_border,
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

    /// Set the color for focused elements (secondary)
    pub fn with_focused(mut self, clr: Color) -> Self {
        self.focused = clr;
        self
    }

    /// Set color for interacting elements (tertiary accent)
    pub fn with_interacting(mut self, clr: Color) -> Self {
        self.interacting = clr;
        self
    }

    /// Get text style
    pub fn style(&self, group: StyleGroup) -> TextStyle {
        let style = TextStyle::default().with_background(self.background);
        match group {
            StyleGroup::Disabled => style.with_foreground(self.light_shadow),
            StyleGroup::Primary => style.with_foreground(self.primary),
            StyleGroup::Hovered => style.with_foreground(self.interacting),
            StyleGroup::Focused => style
                .with_foreground(self.focused)
                .with_appearance(Appearance::default().with_reverse(true)),
            StyleGroup::Interacted => style
                .with_foreground(self.interacting)
                .with_appearance(Appearance::default().with_reverse(true)),
            StyleGroup::LightShadow => style.with_foreground(self.light_shadow),
            StyleGroup::DarkShadow => style.with_foreground(self.dark_shadow),
            _ => style.with_foreground(self.foreground),
        }
    }

    /// Get the border style
    pub fn border_style(&self, group: WidgetGroup) -> BorderStyle {
        match group {
            WidgetGroup::Normal => self.normal_border,
            WidgetGroup::Button => self.button_border,
        }
    }
}
