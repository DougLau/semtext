// widget/mod.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//! User Interface Widgets
use crate::{AreaBound, Cells, Result};

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

mod border;
mod label;
mod linestyle;
mod spacer;

pub use border::{Border, Edge};
pub use label::Label;
pub use linestyle::LineStyle;
pub use spacer::Spacer;
