// widget.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Cells, Result};

pub trait Widget {
    /// Render the widget
    fn render(&self, _cells: &mut Cells) -> Result<()> {
        // default implementation renders nothing
        Ok(())
    }
}
