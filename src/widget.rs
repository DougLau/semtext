// widget.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Cells, Constraints, Result};

/// A component of a user interface
pub trait Widget {
    /// Get the layout constraints
    fn constraints(&self) -> Constraints {
        Constraints::default()
    }

    /// Render the widget
    fn render(&self, _cells: &mut Cells) -> Result<()> {
        // default implementation renders nothing
        Ok(())
    }
}
