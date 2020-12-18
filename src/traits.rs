// traits.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::layout::{AreaBound, Cells};
use crate::widget::Border;
use crate::Result;

/// A component of a user interface
pub trait Widget {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        AreaBound::default()
    }

    /// Get the border
    fn border(&self) -> Option<Border> {
        None
    }

    /// Render the widget
    fn render(&self, _cells: &mut Cells) -> Result<()> {
        // default implementation renders nothing
        Ok(())
    }
}
