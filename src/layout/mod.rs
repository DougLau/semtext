// layout/mod.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//! Layouts

mod bbox;
mod bounds;
mod cells;
mod gridarea;

pub use bbox::BBox;
pub(crate) use bbox::Dim;
pub use bounds::AreaBound;
use bounds::LengthBound;
pub use cells::Cells;
pub use gridarea::{GridArea, GridItem};
