// lib.rs      semtext crate.
//
// Copyright (c) 2020  Douglas Lau
//

mod bbox;
mod border;
mod error;
mod label;
mod screen;
mod widget;

pub use crate::bbox::{BBox, Dim, Edge, Pos};
pub use crate::border::{Border, LineStyle};
pub use crate::error::{Error, Result};
pub use crate::label::Label;
pub use crate::screen::{Cells, Glyph, Screen};
pub use crate::widget::Widget;
