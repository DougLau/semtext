// widget.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Cells, AreaBound, Result};

/// A component of a user interface
pub trait Widget {
    /// Get the area bounds
    fn bounds(&self) -> AreaBound {
        AreaBound::default()
    }

    /// Render the widget
    fn render(&self, _cells: &mut Cells) -> Result<()> {
        // default implementation renders nothing
        Ok(())
    }
}
