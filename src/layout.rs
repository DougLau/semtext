// layout.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//!
use crate::{BBox, Constraints, Constraints1, Error, Result, Widget};

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
        let wcons: Vec<Constraints> =
            self.widgets.iter().map(|w| w.constraints()).collect();
        let col_cons = self.calculate_column_constraints(&wcons[..]);
        let row_cons = self.calculate_row_constraints(&wcons[..]);
        // Distribute excess cells from lyr BBox to columns and rows
        /*
        for row in &row_cons {
            for col in &col_cons {
                cell_boxes.push(Constraints::new(*col, *row));
            }
        }*/
        let mut cell_boxes = vec![];
        Ok(cell_boxes)
    }

    fn calculate_column_constraints(
        &self,
        wcons: &[Constraints],
    ) -> Vec<Constraints1> {
        let mut col_cons = vec![Constraints1::default(); self.cols.into()];
        let mut done = 0; // number of widgets completed
        let mut width = 1;
        while done < wcons.len() && width <= self.cols {
            for (con, gb) in wcons.iter().zip(&self.grid_boxes) {
                if gb.width() == width {
                    let start = gb.left().into();
                    let end = gb.right().into();
                    let mut cons = &mut col_cons[start..end];
                    adjust_constraints(&mut cons, con.col);
                    done += 1;
                }
            }
            width += 1;
        }
        col_cons
    }

    fn calculate_row_constraints(
        &self,
        wcons: &[Constraints],
    ) -> Vec<Constraints1> {
        let mut row_cons = vec![Constraints1::default(); self.rows.into()];
        let mut done = 0;
        let mut height = 1;
        while done < wcons.len() && height <= self.rows {
            for (con, gb) in wcons.iter().zip(&self.grid_boxes) {
                if gb.height() == height {
                    let start = gb.top().into();
                    let end = gb.bottom().into();
                    let mut cons = &mut row_cons[start..end];
                    adjust_constraints(&mut cons, con.row);
                    done += 1;
                }
            }
            height += 1;
        }
        row_cons
    }
}

/// Check if two widgets are at the same memory address
fn widget_is_same(a: &dyn Widget, b: &dyn Widget) -> bool {
    a as *const _ == b as *const _
}

/// Adjust cell constraints for one dimension
///
/// * `cons`: Constraints for all columns or rows
/// * `wcon`: Widget constraints
fn adjust_constraints(cons: &mut [Constraints1], wcon: Constraints1) {
    distribute_decrease(cons, wcon.maximum());
    let min: u16 = cons.iter().map(|c| c.minimum()).sum();
    if min < wcon.minimum() {
        let increase = wcon.minimum() - min;
        let push_increase = distribute_increase(cons, increase, false);
        distribute_increase(cons, push_increase, true);
    }
}

/// Decrease maximums on a slice of constraints
fn distribute_decrease(con_slice: &mut [Constraints1], maximum: u16) {
    let mut unbounded = 0; // count of unbounded constraints
    let mut total = 0; // total of bounded maximum constraints
    for con in con_slice.iter_mut() {
        let max = con.maximum();
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
    for con in con_slice.iter_mut() {
        let max = con.maximum();
        if max == u16::MAX {
            if bonus > 0 {
                con.decrease(each + 1);
                bonus -= 1;
            } else {
                con.decrease(each);
            }
        }
    }
}

/// Increase minimums on a slice of constraints
fn distribute_increase(
    con_slice: &mut [Constraints1],
    increase: u16,
    push: bool,
) -> u16 {
    let mut increase = increase;
    while increase > 0 {
        let before = increase;
        for con in con_slice.iter_mut() {
            if push || con.available() > 0 {
                con.increase();
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
