// widget.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Error, Grid, Result};

pub trait Widget {
    /// Render the widget onto a grid
    fn render(&self, grid: &mut Grid) -> Result<Error>;
}
