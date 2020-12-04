// layout.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//!
use crate::{AreaBound, BBox, Error, LengthBound, Result, Widget};

/// Builder for widget layouts.
struct LayoutBuilder<'a> {
    /// `Widget` references, laid out in row-major order
    grid: Vec<&'a dyn Widget>,
    /// Grid rows
    rows: u16,
    /// Grid columns
    cols: u16,
    /// `Widget` references, with no duplicates
    widgets: Vec<&'a dyn Widget>,
    /// Grid bounding boxes for all widgets
    grid_boxes: Vec<BBox>,
}

/// Widget layout
pub struct Layout<'a> {
    /// All widgets in layout
    widgets: Vec<&'a dyn Widget>,
    /// Cell bounding boxes for all widgets
    boxes: Vec<BBox>,
}

impl<'a> LayoutBuilder<'a> {
    /// Create a new layout builder
    fn new<'b>(grid: Vec<&'a dyn Widget>, rows: u16) -> Result<Self> {
        let len = grid.len() as u16; // FIXME
        let cols = len / rows;
        if cols * rows != len {
            return Err(Error::InvalidLayout());
        }
        let mut builder = LayoutBuilder {
            grid,
            rows,
            cols,
            widgets: vec![],
            grid_boxes: vec![],
        };
        builder.widgets = builder.widgets_unique();
        builder.grid_boxes = builder.calculate_grid_boxes()?;
        Ok(builder)
    }

    /// Make a `Vec` of unique widgets
    fn widgets_unique(&self) -> Vec<&'a dyn Widget> {
        let mut widgets = Vec::new();
        for widget in &self.grid {
            if !widgets.iter().any(|w| widget_is_same(*w, *widget)) {
                widgets.push(*widget);
            }
        }
        widgets
    }

    /// Calculate widget bounding boxes in grid units
    fn calculate_grid_boxes(&self) -> Result<Vec<BBox>> {
        let mut grid_boxes = Vec::new();
        for widget in &self.widgets {
            grid_boxes.push(self.widget_grid_bbox(*widget)?);
        }
        Ok(grid_boxes)
    }

    /// Get the widget bounding box in grid units
    fn widget_grid_bbox(&self, widget: &dyn Widget) -> Result<BBox> {
        let mut top = u16::MAX;
        let mut bottom = u16::MIN;
        let mut left = u16::MAX;
        let mut right = u16::MIN;
        let mut count = 0;
        for (i, w) in self.grid.iter().enumerate() {
            if widget_is_same(*w, widget) {
                let row = i as u16 / self.cols;
                top = top.min(row);
                bottom = bottom.max(row);
                let col = i as u16 % self.cols;
                left = left.min(col);
                right = right.max(col);
                count += 1;
            }
        }
        if count > 0 {
            let width = right - left + 1;
            let height = bottom - top + 1;
            if count == width * height {
                return Ok(BBox::new(left, top, width, height));
            }
        }
        Err(Error::InvalidLayout())
    }

    /// Build the layout
    fn build(self, lyr: BBox) -> Result<Layout<'a>> {
        let boxes = self.calculate_cell_boxes(lyr)?;
        let widgets = self.widgets;
        Ok(Layout { widgets, boxes })
    }

    /// Calculate cell bounding boxes
    fn calculate_cell_boxes(&self, lyr: BBox) -> Result<Vec<BBox>> {
        let w_bounds: Vec<AreaBound> =
            self.widgets.iter().map(|w| w.bounds()).collect();
        let col_bounds = self.calculate_column_bounds(&w_bounds[..]);
        let row_bounds = self.calculate_row_bounds(&w_bounds[..]);
        // Distribute excess cells from lyr BBox to columns and rows
        /*
        for row in &row_cons {
            for col in &col_cons {
                cell_boxes.push(AreaBound::new(*col, *row));
            }
        }*/
        let mut cell_boxes = vec![];
        Ok(cell_boxes)
    }

    /// Calculate bounds for all columns
    fn calculate_column_bounds(
        &self,
        w_bounds: &[AreaBound],
    ) -> Vec<LengthBound> {
        let mut col_bounds = vec![LengthBound::default(); self.cols.into()];
        let mut done = 0; // number of widgets completed
        let mut width = 1;
        while done < w_bounds.len() && width <= self.cols {
            for (wbnd, gb) in w_bounds.iter().zip(&self.grid_boxes) {
                if gb.width() == width {
                    let start = gb.left().into();
                    let end = gb.right().into();
                    let mut bounds = &mut col_bounds[start..end];
                    adjust_length_bounds(&mut bounds, wbnd.col);
                    done += 1;
                }
            }
            width += 1;
        }
        col_bounds
    }

    /// Calculate bounds for all rows
    fn calculate_row_bounds(&self, w_bounds: &[AreaBound]) -> Vec<LengthBound> {
        let mut row_bounds = vec![LengthBound::default(); self.rows.into()];
        let mut done = 0;
        let mut height = 1;
        while done < w_bounds.len() && height <= self.rows {
            for (wbnd, gb) in w_bounds.iter().zip(&self.grid_boxes) {
                if gb.height() == height {
                    let start = gb.top().into();
                    let end = gb.bottom().into();
                    let mut bounds = &mut row_bounds[start..end];
                    adjust_length_bounds(&mut bounds, wbnd.row);
                    done += 1;
                }
            }
            height += 1;
        }
        row_bounds
    }
}

/// Check if two widgets are at the same memory address
fn widget_is_same(a: &dyn Widget, b: &dyn Widget) -> bool {
    a as *const _ == b as *const _
}

/// Adjust a slice of length bounds to match a widget's bounds
///
/// * `bounds`: Length bounds for columns or rows containing the widget
/// * `wbnd`: Length bounds for the widget
fn adjust_length_bounds(bounds: &mut [LengthBound], wbnd: LengthBound) {
    distribute_decrease(bounds, wbnd.maximum());
    let min: u16 = bounds.iter().map(|c| c.minimum()).sum();
    if min < wbnd.minimum() {
        let increase = wbnd.minimum() - min;
        let push_increase = distribute_increase(bounds, increase, false);
        distribute_increase(bounds, push_increase, true);
    }
}

/// Decrease maximums on a slice of length bounds
fn distribute_decrease(bounds: &mut [LengthBound], maximum: u16) {
    let mut unbounded = 0; // count of unbounded lengths
    let mut total = 0; // total of bounded maximum lengths
    for bnd in bounds.iter_mut() {
        let max = bnd.maximum();
        if max < u16::MAX {
            total += max;
        } else {
            unbounded += 1;
        }
    }
    let extra = if maximum > total { maximum - total } else { 0 };
    let (each, bonus) = if extra > 0 && unbounded > 0 {
        (extra / unbounded, extra % unbounded)
    } else {
        (0, 0)
    };
    let mut bonus = bonus;
    for bnd in bounds.iter_mut() {
        let max = bnd.maximum();
        if max == u16::MAX {
            if bonus > 0 {
                bnd.decrease(each + 1);
                bonus -= 1;
            } else {
                bnd.decrease(each);
            }
        }
    }
}

/// Increase minimums on a slice of length bounds
fn distribute_increase(
    bounds: &mut [LengthBound],
    increase: u16,
    push: bool,
) -> u16 {
    let mut increase = increase;
    while increase > 0 {
        let before = increase;
        for bnd in bounds.iter_mut() {
            if push || bnd.available() > 0 {
                bnd.increase(1);
                increase -= 1;
                if increase == 0 {
                    break;
                }
            }
        }
        if increase == before {
            break;
        }
    }
    increase
}

impl<'a> Layout<'a> {
    /// Create a new widget layout
    ///
    /// Rather than using this function directly, the [layout] macro is
    /// recommended.
    ///
    /// * `bbox`: Bounding box of layout.
    /// * `grid`: A slice of `Widget` references, laid out in row-major order.
    /// * `rows`: The number of rows in the grid.
    pub fn new(
        bbox: BBox,
        grid: Vec<&'a dyn Widget>,
        rows: u16,
    ) -> Result<Self> {
        LayoutBuilder::new(grid, rows)?.build(bbox)
    }
}

/// Layout [Widget]s onto a grid
///
/// * `bbox`: Bounding box of cells to lay out
/// * `[a ...], [b ...],`: One or more rows of `Widget`s.  A row is a series of
///                        identifiers, separated by spaces and enclosed in
///                        square brackets, ending with a comma.
///
/// * returns: `Result<`[Layout]`>`
///
/// A `Widget` identifier can appear multiple times as long as it occupies a
/// rectangular shape in the grid.
///
/// This macro is inspired by the concise CSS [grid-template-areas] property.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate semtext;
/// # fn main() {
/// use semtext::{BBox, Label, Layout, Widget};
///
/// let a = Label::new("Top Left");
/// let b = Label::new("Right");
/// let c = Label::new("Bottom Left");
/// let bbox = BBox::new(0, 0, 80, 25);
/// let g = layout!(bbox,
///     [a a b],
///     [a a b],
///     [c c b],
/// ).unwrap();
/// # }
/// ```
/// [grid-template-areas]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-areas
#[macro_export]
macro_rules! layout {
    ( $bbox:expr, $([ $($widget:ident)+ ],)+ ) => {
        {
            let mut g = Vec::<&dyn Widget>::new();
            let mut rows = 0;
            $(
                $( g.push(&$widget); )+
                rows += 1;
            )?
            Layout::new($bbox, g, rows)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Label;

    #[test]
    fn grids() {
        let a = Label::new("Label A");
        let b = Label::new("Label B");
        let c = Label::new("Label C");
        let bbox = BBox::new(0, 0, 80, 25);
        let g = layout!(
            bbox,
            [a a b b],
            [a a c c],
            [a a c c],
        )
        .unwrap();
        assert!(widget_is_same(g.widgets[0], &a));
        assert!(widget_is_same(g.widgets[1], &b));
        assert!(widget_is_same(g.widgets[2], &c));
    }
}
