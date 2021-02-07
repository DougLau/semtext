// scrollview.rs
//
// Copyright (c) 2020-2021  Douglas P Lau
//
use crate::input::{Action, FocusEvent, ModKeys, MouseEvent};
use crate::layout::{BBox, Cells, Dim, LengthBound, Pos};
use crate::text::{StyleGroup, Theme};
use crate::{Result, Widget};
use std::cell::Cell;

/// Scroll view state
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    /// Scroll view disabled
    Disabled,
    /// Scroll view enabled
    Enabled,
    /// Scroll view focused
    Focused,
    /// Scroll view held
    Held,
}

/// Scroll bar configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollBar {
    /// Vertical scroll bar
    Vertical(u16),
    /// Horizontal scroll bar
    Horizontal(u16),
    /// Vertical and horizontal scroll bars
    VerticalAndHorizontal(u16, u16),
}

/// Vertical scroll bar widget
struct VerticalScrollBar {
    /// Scroll view rows
    rows: u16,
    /// Scroll bar state
    state: Cell<State>,
    /// Wrapped widget height
    height: Cell<u16>,
}

/// Horizontal scroll bar widget
struct HorizontalScrollBar {
    /// Scroll view columns
    cols: u16,
    /// Scroll bar state
    state: Cell<State>,
    /// Wrapped widget width
    width: Cell<u16>,
}

/// Scroll view widget wrapper
pub struct ScrollView<W: Widget> {
    /// Wrapped widget
    wrapped: W,
    /// Vertical scroll bar
    v_bar: Option<VerticalScrollBar>,
    /// Horizontal scroll bar
    h_bar: Option<HorizontalScrollBar>,
    /// Offset within wrapped widget
    offset: Cell<Pos>,
    /// Widget state
    state: Cell<State>,
}

impl VerticalScrollBar {
    fn new(rows: u16) -> Self {
        let state = Cell::new(State::Enabled);
        let height = Cell::new(0);
        Self {
            rows,
            state,
            height,
        }
    }

    /// Get the start and end rows of the thumb
    ///
    /// * `bar_height`: Scroll bar height
    /// * `pos`: Position within wrapped widget
    fn thumb_rows(&self, bar_height: u16, pos: Pos) -> (u16, u16) {
        let height = self.height.get();
        let tfrac = f32::from(bar_height) / f32::from(height);
        let start = (f32::from(pos.row) * tfrac).ceil() as u16;
        let end = (f32::from(pos.row + bar_height) * tfrac).floor() as u16;
        (start, end)
    }
}

impl Widget for VerticalScrollBar {
    /// Get the style group
    fn style_group(&self) -> StyleGroup {
        match self.state.get() {
            State::Disabled => StyleGroup::Disabled,
            State::Enabled => StyleGroup::Enabled,
            State::Focused => StyleGroup::Focused,
            State::Held => StyleGroup::Interacted,
        }
    }

    /// Get the width bounds
    fn width_bounds(&self, _theme: &Theme) -> LengthBound {
        LengthBound::new(1..=1)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, pos: Pos) -> Result<()> {
        let height = self.height.get();
        let bar_height = cells.height();
        if height > 0 && bar_height <= height {
            let (start, end) = self.thumb_rows(bar_height, pos);
            for row in 0..bar_height {
                cells.move_to(0, row)?;
                if row < start || row > end {
                    cells.print_char('▓')?;
                } else {
                    cells.print_char('░')?;
                }
            }
        }
        Ok(())
    }

    /// Handle mouse events
    fn mouse_event(
        &self,
        mev: MouseEvent,
        _mods: ModKeys,
        _dim: Dim,
        _pos: Pos,
    ) -> Option<Action> {
        let state = self.state.get();
        match (mev, state) {
            (_, State::Disabled) => None,
            (MouseEvent::ButtonDown(_), _) => Some(State::Held),
            _ => None,
        }
        .and_then(|st| {
            if st != state {
                self.state.set(st);
                Some(Action::Redraw())
            } else {
                None
            }
        })
    }
}

impl HorizontalScrollBar {
    fn new(cols: u16) -> Self {
        let state = Cell::new(State::Enabled);
        let width = Cell::new(0);
        Self { cols, state, width }
    }
}

impl Widget for HorizontalScrollBar {
    /// Get the height bounds
    fn height_bounds(&self, _theme: &Theme, _width: u16) -> LengthBound {
        LengthBound::new(1..=1)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, pos: Pos) -> Result<()> {
        assert_eq!(pos, Pos::default(), "FIXME");
        cells.move_to(0, 0)?;
        for _ in 0..cells.width() {
            cells.print_char('░')?;
        }
        Ok(())
    }
}

impl<W: Widget> ScrollView<W> {
    /// Create a new scroll view
    pub fn new(wrapped: W) -> Self {
        let v_bar = Some(VerticalScrollBar::new(8));
        let h_bar = None;
        let offset = Cell::new(Pos::default());
        let state = Cell::new(State::Enabled);
        Self {
            wrapped,
            v_bar,
            h_bar,
            offset,
            state,
        }
    }

    /// Configure scroll bars
    pub fn with_bars(mut self, bars: ScrollBar) -> Self {
        match bars {
            ScrollBar::Vertical(rows) => {
                self.v_bar = Some(VerticalScrollBar::new(rows));
                self.h_bar = None;
            }
            ScrollBar::Horizontal(cols) => {
                self.v_bar = None;
                self.h_bar = Some(HorizontalScrollBar::new(cols));
            }
            ScrollBar::VerticalAndHorizontal(rows, cols) => {
                self.v_bar = Some(VerticalScrollBar::new(rows));
                self.h_bar = Some(HorizontalScrollBar::new(cols));
            }
        }
        self
    }

    /// Get the wrapped widget
    pub fn wrapped(&self) -> &W {
        &self.wrapped
    }

    /// Set the widget state
    fn set_state(&self, st: State) -> Option<Action> {
        let mut action = None;
        if st != self.state.get() {
            self.state.set(st);
            action = Some(Action::Redraw());
        }
        if let Some(v_bar) = &self.v_bar {
            if st != v_bar.state.get() {
                v_bar.state.set(st);
                action = Some(Action::Redraw());
            }
        }
        if let Some(h_bar) = &self.h_bar {
            if st != h_bar.state.get() {
                h_bar.state.set(st);
                action = Some(Action::Redraw());
            }
        }
        action
    }
}

impl<W: Widget> Widget for ScrollView<W> {
    /// Get the width bounds
    fn width_bounds(&self, theme: &Theme) -> LengthBound {
        let mut bounds = self.wrapped.width_bounds(theme);
        if let Some(v_bar) = &self.v_bar {
            bounds = bounds + v_bar.width_bounds(theme);
        }
        let mut min_col = bounds.minimum();
        if let Some(h_bar) = &self.h_bar {
            h_bar.width.set(bounds.minimum());
            min_col = min_col.min(h_bar.cols);
        }
        let max_col = bounds.maximum();
        LengthBound::new(min_col..=max_col)
    }

    /// Get the height bounds
    fn height_bounds(&self, theme: &Theme, width: u16) -> LengthBound {
        let mut bounds = self.wrapped.height_bounds(theme, width);
        if let Some(h_bar) = &self.h_bar {
            bounds = bounds + h_bar.height_bounds(theme, width);
        }
        let mut min_row = bounds.minimum();
        if let Some(v_bar) = &self.v_bar {
            v_bar.height.set(bounds.minimum());
            min_row = min_row.min(v_bar.rows);
        }
        let max_row = bounds.maximum();
        LengthBound::new(min_row..=max_row)
    }

    /// Draw the widget
    fn draw(&self, cells: &mut Cells, offset: Pos) -> Result<()> {
        assert_eq!(offset, Pos::default(), "FIXME");
        let offset = self.offset.get();
        let mut width = cells.width();
        let mut height = cells.height();
        if width == 0 || height == 0 {
            return Ok(());
        }
        if self.v_bar.is_some() {
            width -= 1;
        }
        if self.h_bar.is_some() {
            height -= 1;
        }
        let width_bounds = self.wrapped.width_bounds(cells.theme());
        let height_bounds = self
            .wrapped
            .height_bounds(cells.theme(), width_bounds.minimum());
        if height_bounds.minimum() <= height && width_bounds.minimum() <= width
        {
            self.set_state(State::Disabled);
        }
        let w_style = cells.theme().style(self.wrapped.style_group());
        if let Some(v_bar) = &self.v_bar {
            let style = cells.theme().style(v_bar.style_group());
            cells.clip(Some(BBox::new(width, 0, 1, height)));
            cells.set_style(style)?;
            v_bar.draw(cells, offset)?;
        }
        if let Some(h_bar) = &self.h_bar {
            let style = cells.theme().style(h_bar.style_group());
            cells.clip(Some(BBox::new(0, height, width, 1)));
            cells.set_style(style)?;
            h_bar.draw(cells, offset)?;
        }
        cells.clip(Some(BBox::new(0, 0, width, height)));
        cells.set_style(w_style)?;
        self.wrapped.draw(cells, offset)
    }

    /// Handle focus event
    fn focus(&self, fev: FocusEvent) -> Option<Action> {
        use FocusEvent::*;
        use State::*;
        let state = self.state.get();
        let act = match (fev, state) {
            (_, Disabled) => Some(Disabled),
            (Offer, _) => Some(Focused),
            (Take, _) => Some(Enabled),
            (HoverOutside, Held) | (HoverInside, Held) => Some(Focused),
            _ => None,
        }
        .and_then(|st| self.set_state(st));
        self.wrapped.focus(fev).or(act)
    }

    /// Handle mouse events
    fn mouse_event(
        &self,
        mev: MouseEvent,
        mods: ModKeys,
        mut dim: Dim,
        pos: Pos,
    ) -> Option<Action> {
        if let Some(v_bar) = &self.v_bar {
            if dim.width > 0 {
                if pos.col >= dim.width - 1 {
                    dim = Dim::new(1, dim.height);
                    let pos = Pos::new(0, pos.row);
                    let act = v_bar.mouse_event(mev, mods, dim, pos);
                    if act.is_some() {
                        let offset = self.offset.get();
                        let (start, end) = v_bar.thumb_rows(dim.height, offset);
                        let mut row = offset.row;
                        if pos.row < start {
                            row -= 1;
                        } else if pos.row > end {
                            row += 1;
                        }
                        self.offset.set(Pos::new(offset.col, row));
                        self.set_state(v_bar.state.get());
                    }
                    return act;
                } else {
                    dim = Dim::new(dim.width - 1, dim.height);
                }
            }
        }
        if let Some(h_bar) = &self.h_bar {
            if dim.height > 0 {
                if pos.row >= dim.height - 1 {
                    dim = Dim::new(dim.width, 1);
                    let pos = Pos::new(pos.col, 0);
                    let act = h_bar.mouse_event(mev, mods, dim, pos);
                    if act.is_some() {
                        self.set_state(h_bar.state.get());
                    }
                    return act;
                } else {
                    dim = Dim::new(dim.width, dim.height - 1);
                }
            }
        }
        self.wrapped
            .mouse_event(mev, mods, dim, self.offset.get() + pos)
    }
}
