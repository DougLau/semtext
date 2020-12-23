// button.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, Cells};
use crate::text::Outline;
use crate::widget::{Border, Label};
use crate::{Result, Widget};

/// Button widget
pub struct Button {
    lbl: Label,
}

impl Button {
    /// Create a new button widget
    pub fn new(txt: &str) -> Self {
        let lbl = Label::new(txt);
        Self { lbl }
    }
}

impl Widget for Button {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        self.lbl.bounds()
    }

    /// Get the border
    fn border(&self) -> Option<Border> {
        Some(Border::default().with_outline(Outline::Solid))

    /// Draw the widget
    fn draw(&self, cells: &mut Cells) -> Result<()> {
        let theme = cells.theme();
        let style = self.style(theme);
        cells.set_style(style)?;
        cells.print_text(self.lbl.txt())
    }

    }
}
