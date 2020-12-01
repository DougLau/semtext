// grid.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::{Error, Result, Widget};

/// A helper to lay out widgets.
///
/// This is used to lay out `Widget`s in a rectangular layer of cells.
pub struct Grid<'a> {
    widgets: Vec<&'a dyn Widget>,
    rows: usize,
    cols: usize,
}

impl<'a> Grid<'a> {
    /// Create a new layout grid
    ///
    /// Rather than using this function directly, the `grid` macro is
    /// recommended.
    ///
    /// * `widgets`: A `Vec` of references to `Widget`s, laid out in column,
    ///              then row order.
    /// * `rows`: The number of rows in the grid.
    pub fn new(widgets: Vec<&'a dyn Widget>, rows: usize) -> Result<Self> {
        let cols = widgets.len() / rows;
        if cols * rows != widgets.len() {
            return Err(Error::InvalidGrid());
        }
        for widget in &widgets {
            Grid::check_rectangular(*widget, &widgets, cols)?;
        }
        Ok(Grid {
            widgets,
            rows,
            cols,
        })
    }

    /// Check that a width has a rectangular layout
    fn check_rectangular(
        widget: &dyn Widget,
        widgets: &[&dyn Widget],
        cols: usize,
    ) -> Result<()> {
        let mut top = usize::MAX;
        let mut bottom = usize::MIN;
        let mut left = usize::MAX;
        let mut right = usize::MIN;
        let mut count = 0;
        for (i, w) in widgets.iter().enumerate() {
            if Grid::is_same(*w, widget) {
                let row = i / cols;
                top = top.min(row);
                bottom = bottom.max(row);
                let col = i % cols;
                left = left.min(col);
                right = right.max(col);
                count += 1;
            }
        }
        if count > 0 {
            let width = right - left + 1;
            let height = bottom - top + 1;
            if count != width * height {
                return Err(Error::InvalidGrid());
            }
        }
        Ok(())
    }

    /// Check if two widgets are the same (memory address)
    fn is_same(a: &dyn Widget, b: &dyn Widget) -> bool {
        a as *const _ == b as *const _
    }
}

/// Build a [Grid] of [Widget]s
///
/// This macro is inspired by the concise CSS [grid-template-areas] property.
/// Each row is a series of [Widget] identifiers, separated by spaces, and
/// terminated by a comma.
/// A [Widget] can appear multiple times in the grid as long as it occupies a
/// rectangular pattern.
///
/// ## Example
/// ```rust
/// # #[macro_use] extern crate semtext;
/// # fn main() {
/// use semtext::{Grid, Label, Widget};
///
/// let a = Label::new("Top Left");
/// let b = Label::new("Right");
/// let c = Label::new("Bottom Left");
/// let g = grid!(
///     a a b,
///     a a b,
///     c c b,
/// );
/// # }
/// ```
/// [grid-template-areas]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-areas
/// [Grid]: struct.Grid.html
/// [Widget]: trait.Widget.html
#[macro_export]
macro_rules! grid {
    ( $($($widget:ident) + ,)*) => {
        {
            let mut g = Vec::<&dyn Widget>::new();
            let mut rows = 0;
            $(
                $( g.push(&$widget); )+
                rows += 1;
            )?
            Grid::new(g, rows)
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
        let g = grid!(
            a a b b,
            a a c c,
            a a c c,
        )
        .unwrap();
        assert!(Grid::is_same(g.widgets[0], &a));
        assert!(Grid::is_same(g.widgets[2], &b));
        assert!(Grid::is_same(g.widgets[5], &a));
        assert!(Grid::is_same(g.widgets[11], &c));
    }
}
