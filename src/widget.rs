use crate::state::{KeyState, MouseState};
use std::ops::Add;

pub type Position<P> = (P, P);
pub type Dimensions<D> = (D, D);
pub type Boundaries<P, D> = (P, P, D, D);

// BITFLAGS (Sorry for no use the crate)
pub type Flags = u16;
pub type BindID = u32;

const CHANGED: Flags = 0b00000001;
pub mod flags {
    use crate::widget::Flags;

    pub const DRAW: Flags = 0b00000010;
    pub const UPDATE: Flags = 0b00000100;
    pub const VISIBLE: Flags = 0b00001000;
    pub const ENABLED: Flags = 0b00010000;
    pub const HOVER: Flags = 0b00100000;
    pub const GRAB: Flags = 0b01000000;
    pub const FOCUS: Flags = 0b10000000;
}

use flags::{DRAW, FOCUS, VISIBLE};

// TODO: create a check_bind

/// A Widget trait is used for the general methods that can be used on every widget.
pub trait Widget<P: Sized + Copy + Clone, D: Sized + Copy + Clone>
where
    D: PartialOrd + Default,
    P: Sized + Add<Output = P> + PartialOrd + From<D> + Default,
{
    /// Get minimal (i32, i32) of the Widget
    fn compute_min(&self) -> Dimensions<D>;
    /// Draw the widget
    fn draw(&mut self, internal: &WidgetInternal<P, D>) -> bool;
    /// Update the status of the widget
    fn update(&mut self, internal: &mut WidgetInternal<P, D>);
    /// Update the layout of the widget
    fn layout(&mut self, internal: &mut WidgetInternal<P, D>);
    /// Search and Update widgets by Bind ID
    fn bind(&mut self, internal: &mut WidgetInternal<P, D>, bind: BindID);
    /// Handle a mouse state (focus, grab)
    fn handle_mouse(&mut self, internal: &mut WidgetInternal<P, D>, mouse: &MouseState<P>);
    /// Handle a keyboard state
    fn handle_keys(&mut self, internal: &mut WidgetInternal<P, D>, key: &KeyState);
    /// Step the focus
    fn step_focus(&mut self, internal: &mut WidgetInternal<P, D>, _: bool) -> bool {
        let check = !internal.check(FOCUS);
        internal.set(DRAW, check && internal.check(VISIBLE));

        check
    }
    /// When you unhover the widget
    fn unhover(&mut self, internal: &mut WidgetInternal<P, D>) {
        internal.on(DRAW);
    }
    /// When you unfocus the widget
    fn unfocus(&mut self, internal: &mut WidgetInternal<P, D>) {
        internal.on(DRAW);
    }
}

pub struct WidgetInternal<P, D> {
    /// (i32, i32)
    dim: Dimensions<D>,
    /// Minimun dimensions
    min_dim: Dimensions<D>,
    /// Relative position
    rel_pos: Position<P>,
    /// Absolute position
    abs_pos: Position<P>,
    /// Every Widget Flags
    flags: Flags,
    /// ID
    bind_id: u32,
}

impl<P, D> WidgetInternal<P, D> {
    // FLAGS
    pub fn set(&mut self, flag: Flags, value: bool) {
        if value {
            self.flags |= flag | CHANGED;
        } else {
            self.flags = self.flags & !flag | CHANGED;
        }
    }

    #[inline]
    pub fn toggle(&mut self, flag: Flags) {
        self.flags ^= flag;
    }

    #[inline]
    pub fn on(&mut self, flag: Flags) {
        self.flags |= flag | CHANGED;
    }

    #[inline]
    pub fn off(&mut self, flag: Flags) {
        self.flags = self.flags & !flag | CHANGED;
    }

    #[inline]
    pub fn check_any(&self, flag: Flags) -> bool {
        flag & self.flags > 0
    }

    #[inline]
    pub fn check(&self, flag: Flags) -> bool {
        flag & self.flags == flag
    }

    #[inline]
    pub fn changed(&mut self) -> bool {
        let ch = self.flags & CHANGED != 0;
        self.flags &= !CHANGED;

        ch
    }

    #[inline]
    pub fn unchange(&mut self) {
        self.flags &= !CHANGED;
    }

    #[inline]
    pub fn val(&self, flag: Flags) -> Flags {
        flag & self.flags
    }

    #[inline]
    pub fn val_all(&self) -> Flags {
        self.flags
    }

    // BIND ID
    #[inline]
    pub fn bind(&self) -> BindID {
        self.bind_id
    }

    #[inline]
    pub fn check_bind(&self, id: BindID) -> bool {
        self.bind_id == id
    }

    pub fn set_bind(&mut self, id: BindID) {
        self.bind_id = id;
    }
}

impl<P: Sized + Copy + Clone, D: Sized + Copy + Clone> WidgetInternal<P, D>
where
    D: PartialOrd + Default,
    P: Add<Output = P> + PartialOrd + From<D> + Default,
{
    // BOUNDARIES

    pub fn new(flags: Flags, bind_id: u32) -> Self {
        WidgetInternal {
            dim: (Default::default(), Default::default()),
            min_dim: (Default::default(), Default::default()),
            rel_pos: (Default::default(), Default::default()),
            abs_pos: (Default::default(), Default::default()),
            flags,
            bind_id,
        }
    }

    pub fn new_with(rel_pos: Position<P>, dim: Dimensions<D>, flags: Flags, bind_id: u32) -> Self {
        WidgetInternal {
            dim,
            min_dim: (Default::default(), Default::default()),
            rel_pos,
            abs_pos: (Default::default(), Default::default()),
            flags,
            bind_id,
        }
    }

    fn check_min(&mut self) {
        if self.dim.0 < self.min_dim.0 {
            self.dim.0 = self.min_dim.0;
        }
        if self.dim.1 < self.min_dim.1 {
            self.dim.1 = self.min_dim.1;
        }
    }

    /// Set relative position and dimensions with 4 item tuple (x, y, width, height)
    pub fn set_boundaries(&mut self, bounds: Boundaries<P, D>) {
        self.rel_pos = (bounds.0, bounds.1);
        self.dim = (bounds.2, bounds.3);

        self.check_min();
    }

    /// Change position
    pub fn set_position(&mut self, pos: Position<P>) {
        self.rel_pos.0 = pos.0;
        self.rel_pos.1 = pos.1;
    }

    /// Sum absolute position
    pub fn compute_absolute(&mut self, pos: Position<P>) {
        self.abs_pos = (pos.0 + self.rel_pos.0, pos.1 + self.rel_pos.1);
    }

    /// Change dimensions
    pub fn set_dimensions(&mut self, dim: Dimensions<D>) {
        self.dim = dim;

        self.check_min();
    }

    /// Change minimal dimensions
    pub fn set_min_dimensions(&mut self, dim: Dimensions<D>) {
        self.min_dim = dim;

        self.check_min();
    }

    /// Change x coordinate
    pub fn set_x(&mut self, x: P) {
        self.rel_pos.0 = x;
    }

    /// Change y coordinate
    pub fn set_y(&mut self, y: P) {
        self.rel_pos.1 = y;
    }

    /// Change width
    pub fn set_width(&mut self, width: D) {
        self.dim.0 = width;

        self.check_min();
    }

    /// Change height
    pub fn set_height(&mut self, height: D) {
        self.dim.1 = height;

        self.check_min();
    }

    /// Get (i32, i32, i32, i32) with relative position
    #[inline]
    pub fn boundaries_rel(&self) -> Boundaries<P, D> {
        (self.rel_pos.0, self.rel_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get (i32, i32, i32, i32) with absolute position
    pub fn boundaries_abs(&self) -> Boundaries<P, D> {
        (self.abs_pos.0, self.abs_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get coordinates tuple
    pub fn relative_pos(&self) -> Position<P> {
        self.rel_pos
    }

    #[inline]
    pub fn absolute_pos(&self) -> Position<P> {
        self.abs_pos
    }

    #[inline]
    pub fn min_dimensions(&self) -> Dimensions<D> {
        self.min_dim
    }

    #[inline]
    /// Get dimensions tuple
    pub fn dimensions(&self) -> Dimensions<D> {
        self.dim
    }

    #[inline]
    pub fn on_area(&self, cursor: Position<P>) -> bool {
        self.check(VISIBLE)
            && cursor.0 >= self.abs_pos.0
            && cursor.0 <= self.abs_pos.0 + P::from(self.dim.0)
            && cursor.1 >= self.abs_pos.1
            && cursor.1 <= self.abs_pos.1 + P::from(self.dim.1)
    }
}
