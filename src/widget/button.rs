// button.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, Cells};
use crate::style::Outline;
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
    }

    /// Render the widget
    fn render(&self, cells: &mut Cells) -> Result<()> {
        self.lbl.render(cells)
    }
}
