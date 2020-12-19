// gridarea.rs
//
// Copyright (c) 2020  Douglas P Lau
//
//!
use crate::layout::{AreaBound, BBox, LengthBound};
use crate::{Error, Result, Widget};

/// Item in a grid area
pub enum GridItem<'a> {
    /// [Widget] grid item
    Widget(&'a dyn Widget),
    /// [Spacer] grid item
    ///
    /// [Spacer]: ../widget/struct.Spacer.html
    Spacer(Option<u8>),
}

/// Grid area layout
///
/// A layout using a grid area, containing a set of borrowed [Widget]s.
/// It is used when calling [Screen::step].
///
/// [Screen::step]: ../struct.Screen.html#method.step
pub struct GridArea<'a> {
    /// Grid rows
    rows: u16,
    /// Grid columns
    cols: u16,
    /// `Widget` references, with no duplicates
    widgets: Vec<&'a dyn Widget>,
    /// Grid bounding boxes for all widgets
    grid_boxes: Vec<BBox>,
}

impl<'a> GridArea<'a> {
    /// Create a new grid area layout
    ///
    /// Rather than using this function directly, the [grid_area] macro is
    /// recommended.
    ///
    /// * `grid`: A slice of [GridItem]s, laid out in row-major order.
    /// * `rows`: The number of rows in the grid.
    ///
    /// # Errors
    ///
    /// [Error::InvalidGridArea] If the length of `grid` is not a multiple of
    ///                          `rows`, or if any [GridItem] does not form a
    ///                          rectangular pattern.
    pub fn new(grid: &[GridItem<'a>], rows: u16) -> Result<Self> {
        let len = grid.len() as u16; // FIXME
        let cols = len / rows;
        if cols * rows != len {
            return Err(Error::InvalidGridArea());
        }
        let widgets = widgets_unique(grid);
        let mut area = GridArea {
            rows,
            cols,
            widgets,
            grid_boxes: vec![],
        };
        area.grid_boxes = area.calculate_grid_boxes(grid)?;
        Ok(area)
    }

    /// Calculate widget bounding boxes in grid units
    fn calculate_grid_boxes(&self, grid: &[GridItem]) -> Result<Vec<BBox>> {
        let mut grid_boxes = Vec::new();
        for widget in &self.widgets {
            grid_boxes.push(self.widget_grid_bbox(grid, *widget)?);
        }
        Ok(grid_boxes)
    }

    /// Get the widget bounding box in grid units
    fn widget_grid_bbox(
        &self,
        grid: &[GridItem],
        widget: &dyn Widget,
    ) -> Result<BBox> {
        let mut top = u16::MAX;
        let mut bottom = u16::MIN;
        let mut left = u16::MAX;
        let mut right = u16::MIN;
        let mut count = 0;
        for (i, item) in grid.iter().enumerate() {
            if let GridItem::Widget(w) = item {
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
        }
        if count > 0 {
            let width = right - left + 1;
            let height = bottom - top + 1;
            if count == width * height {
                return Ok(BBox::new(left, top, width, height));
            }
        }
        Err(Error::InvalidGridArea())
    }

    /// Calculate bounding boxes for the widgets
    pub(crate) fn widget_boxes(
        &self,
        bbox: BBox,
    ) -> Result<Vec<(&'a dyn Widget, BBox)>> {
        let boxes = self.calculate_cell_boxes(bbox)?;
        let mut wb = vec![];
        for (widget, bbox) in self.widgets.iter().zip(boxes) {
            wb.push((*widget, bbox));
        }
        Ok(wb)
    }

    /// Calculate cell bounding boxes
    fn calculate_cell_boxes(&self, bx: BBox) -> Result<Vec<BBox>> {
        let w_bounds: Vec<AreaBound> =
            self.widgets.iter().map(|w| widget_bounds(*w)).collect();
        let col_bounds = self.col_bounds(&w_bounds[..]);
        let columns = distribute_bounds(col_bounds, bx.width());
        let row_bounds = self.row_bounds(&w_bounds[..]);
        let rows = distribute_bounds(row_bounds, bx.height());
        let cell_boxes: Vec<BBox> = self
            .grid_boxes
            .iter()
            .map(|gb| widget_cell_bbox(bx, *gb, &columns[..], &rows[..]))
            .collect();
        Ok(cell_boxes)
    }

    /// Create bounds for all columns
    fn col_bounds(&self, w_bounds: &[AreaBound]) -> Vec<LengthBound> {
        let mut col_bounds = vec![LengthBound::default(); self.cols.into()];
        let mut done = 0; // number of widgets completed
        let mut grid_width = 1; // widget grid width
        while done < w_bounds.len() && grid_width <= self.cols {
            for (wbnd, gb) in w_bounds.iter().zip(&self.grid_boxes) {
                if gb.width() == grid_width {
                    let start = gb.left().into();
                    let end = gb.right().into();
                    let mut bounds = &mut col_bounds[start..end];
                    adjust_length_bounds(&mut bounds, wbnd.col);
                    done += 1;
                }
            }
            grid_width += 1;
        }
        col_bounds
    }

    /// Create bounds for all rows
    fn row_bounds(&self, w_bounds: &[AreaBound]) -> Vec<LengthBound> {
        let mut row_bounds = vec![LengthBound::default(); self.rows.into()];
        let mut done = 0; // number of widgets completed
        let mut grid_height = 1; // widget grid height
        while done < w_bounds.len() && grid_height <= self.rows {
            for (wbnd, gb) in w_bounds.iter().zip(&self.grid_boxes) {
                if gb.height() == grid_height {
                    let start = gb.top().into();
                    let end = gb.bottom().into();
                    let mut bounds = &mut row_bounds[start..end];
                    adjust_length_bounds(&mut bounds, wbnd.row);
                    done += 1;
                }
            }
            grid_height += 1;
        }
        row_bounds
    }
}

/// Make a `Vec` of unique widgets
fn widgets_unique<'a>(grid: &[GridItem<'a>]) -> Vec<&'a dyn Widget> {
    let mut widgets = Vec::new();
    for item in grid {
        match item {
            GridItem::Widget(widget) => {
                if !widgets.iter().any(|w| widget_is_same(*w, *widget)) {
                    widgets.push(*widget);
                }
            }
            GridItem::Spacer(_) => {
                // FIXME: Handle spacing
            }
        }
    }
    widgets
}

/// Check if two widgets are at the same memory address
fn widget_is_same(a: &dyn Widget, b: &dyn Widget) -> bool {
    data_pointer(a) == data_pointer(b)
}

/// Get the data pointer of a trait object
///
/// Taken from [here]
///
/// [here]: https://github.com/rust-lang/rust/issues/27751#issuecomment-336554503
fn data_pointer<T: ?Sized>(t: &T) -> *const u8 {
    t as *const T as *const u8
}

/// Get the bounds of a widget (including border)
fn widget_bounds(w: &dyn Widget) -> AreaBound {
    if let Some(b) = w.border() {
        w.bounds() + b.bounds()
    } else {
        w.bounds()
    }
}

/// Adjust a slice of length bounds to match a widget's bounds
///
/// * `bounds`: Length bounds for columns or rows containing the widget
/// * `wbnd`: Length bounds for the widget
fn adjust_length_bounds(bounds: &mut [LengthBound], wbnd: LengthBound) {
    let max = wbnd.maximum();
    if max < u16::MAX {
        distribute_decrease(bounds, max);
    }
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

/// Distribute total lengths to a `Vec` of lengths
///
/// NOTE: this uses a woefully inefficient algorithm
fn distribute_bounds(mut bounds: Vec<LengthBound>, total: u16) -> Vec<u16> {
    let minimum = bounds[..].iter().map(|b| b.minimum()).sum::<u16>();
    if minimum < total {
        let maximum = bounds[..]
            .iter()
            .map(|b| b.maximum())
            .fold(0u16, |sum, b| sum.saturating_add(b));
        let maximum = total.min(maximum);
        let extra = maximum - minimum;
        let mut added = 0;
        while added < extra {
            // find index of bound with max available
            let (i, _) = bounds[..]
                .iter()
                .enumerate()
                .max_by_key(|&(_, &b)| b.available())
                .unwrap();
            bounds[i].increase(1);
            added += 1;
        }
    }
    bounds[..].iter().map(|b| b.minimum()).collect()
}

/// Calculate a widget cell bounding box from grid data
fn widget_cell_bbox(bx: BBox, gb: BBox, cols: &[u16], rows: &[u16]) -> BBox {
    let col = bx.left() + cols[..gb.left() as usize].iter().sum::<u16>();
    let row = bx.top() + rows[..gb.top() as usize].iter().sum::<u16>();
    let width = cols[gb.left() as usize..gb.right() as usize].iter().sum();
    let height = rows[gb.top() as usize..gb.bottom() as usize].iter().sum();
    BBox::new(col, row, width, height)
}

/// Lay out [Widget]s onto a grid area
///
/// This macro is inspired by the concise CSS [grid-template-areas] property.
/// It can be used to construct a [GridArea].
///
/// ## Arguments
///
/// * `[a …] [b …]`: One or more rows of grid items, enclosed in square
///                  brackets.  A grid item is either a [Widget] identifier or a
///                  dot `.`, which is used for spacing.  A `Widget` can appear
///                  multiple times as long as it occupies a rectangular shape
///                  in the grid.
///
/// ## Errors
///
/// [Error::InvalidGridArea] If the length of `grid` is not a multiple of
///                          `rows`, or if any [Widget] does not form a
///                          rectangular pattern.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate semtext;
/// # fn main() {
/// use semtext::{Widget, widget::Label};
///
/// let a = Label::new("Top Left");
/// let b = Label::new("Right");
/// let c = Label::new("Bottom Left");
/// let l = grid_area!(
///     [a a b]
///     [a a b]
///     [c c .]
/// ).unwrap();
/// # }
/// ```
/// [grid-template-areas]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-areas
#[macro_export]
macro_rules! grid_area {
    (.) => { $crate::layout::GridItem::Spacer(None) };
    ($widget:ident) => { $crate::layout::GridItem::Widget(&$widget) };
    ($([ $($item:tt)+ ])+) => {
        {
            let mut ga = Vec::<$crate::layout::GridItem>::new();
            let mut rows = 0;
            $(
                $( ga.push(grid_area!( $item )); )+
                rows += 1;
            )?
            $crate::layout::GridArea::new(&ga[..], rows)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widget::{Label, Spacer};

    #[test]
    fn spacer1() {
        let a = Spacer::default();
        let b = Spacer::default();
        let l = grid_area!([a][b])
            .unwrap()
            .widget_boxes(BBox::new(0, 0, 80, 25))
            .unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[0].1, BBox::new(0, 0, 80, 12));
        assert_eq!(l[1].1, BBox::new(0, 12, 80, 13));
    }

    #[test]
    fn spacer2() {
        let a = Spacer::default();
        let b = Spacer::default();
        let l = grid_area!(
            [a b]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[0].1, BBox::new(0, 0, 40, 25));
        assert_eq!(l[1].1, BBox::new(40, 0, 40, 25));
    }

    #[test]
    fn spacer3() {
        let a = Spacer::default();
        let b = Spacer::default();
        let c = Spacer::default();
        let l = grid_area!(
            [a b]
            [a c]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert!(widget_is_same(l[0].0, &a));
        assert!(widget_is_same(l[1].0, &b));
        assert!(widget_is_same(l[2].0, &c));
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].1, BBox::new(0, 0, 40, 25));
        assert_eq!(l[1].1, BBox::new(40, 0, 40, 12));
        assert_eq!(l[2].1, BBox::new(40, 12, 40, 13));
    }

    #[test]
    fn spacer4() {
        let a = Spacer::default();
        let b = Spacer::default();
        let c = Spacer::default();
        let l = grid_area!(
            [a a b]
            [a a c]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert!(widget_is_same(l[0].0, &a));
        assert!(widget_is_same(l[1].0, &b));
        assert!(widget_is_same(l[2].0, &c));
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].1, BBox::new(0, 0, 53, 25));
        assert_eq!(l[1].1, BBox::new(53, 0, 27, 12));
        assert_eq!(l[2].1, BBox::new(53, 12, 27, 13));
    }

    #[test]
    fn spacer5() {
        let a = Spacer::default();
        let b = Spacer::default();
        let c = Spacer::default();
        let l = grid_area!(
            [a a b b]
            [a a c c]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert!(widget_is_same(l[0].0, &a));
        assert!(widget_is_same(l[1].0, &b));
        assert!(widget_is_same(l[2].0, &c));
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].1, BBox::new(0, 0, 40, 25));
        assert_eq!(l[1].1, BBox::new(40, 0, 40, 12));
        assert_eq!(l[2].1, BBox::new(40, 12, 40, 13));
    }

    #[test]
    fn spacer6() {
        let a = Spacer::default();
        let b = Spacer::default();
        let c = Spacer::default();
        let l = grid_area!(
            [a a b b]
            [a a b b]
            [a a c c]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert!(widget_is_same(l[0].0, &a));
        assert!(widget_is_same(l[1].0, &b));
        assert!(widget_is_same(l[2].0, &c));
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].1, BBox::new(0, 0, 40, 25));
        assert_eq!(l[1].1, BBox::new(40, 0, 40, 16));
        assert_eq!(l[2].1, BBox::new(40, 16, 40, 9));
    }

    #[test]
    fn grid1() {
        let a = Label::new("Label");
        let l = grid_area!([.] [a])
            .unwrap()
            .widget_boxes(BBox::new(0, 0, 80, 25))
            .unwrap();
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].1, BBox::new(0, 24, 9, 1));
    }

    #[test]
    fn grid2() {
        let a = Label::new("Label");
        let l = grid_area!(
            [. a]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].1, BBox::new(74, 0, 6, 2));
    }

    #[test]
    fn grid3() {
        let a = Label::new("Label");
        let l = grid_area!(
            [. .]
            [. a]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].1, BBox::new(74, 24, 6, 1));
    }

    #[test]
    fn grid4() {
        let a = Label::new("This is a test label with some text");
        let b = Label::new("Label");
        let l = grid_area!(
            [. . . .]
            [a . b .]
        )
        .unwrap()
        .widget_boxes(BBox::new(0, 0, 80, 25))
        .unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[0].1, BBox::new(0, 23, 18, 2));
        assert_eq!(l[1].1, BBox::new(46, 23, 6, 2));
    }
}
