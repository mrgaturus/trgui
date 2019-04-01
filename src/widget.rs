//! Structs and Traits for widgets

use crate::signal::{Signal, SignalID};
use crate::state::{KeyState, MouseState};
use std::ops::Add;

pub type Position<P> = (P, P);
pub type Dimensions<D> = (D, D);
pub type Boundaries<P, D> = (P, P, D, D);

// BITFLAGS (Sorry for no use the crate)
pub type Flags = u16;

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

/// Main Widget Trait
pub trait Widget<P: Sized + Copy + Clone, D: Sized + Copy + Clone>
where
    D: PartialOrd + Default,
    P: Sized + Add<Output = P> + PartialOrd + From<D> + Default,
{
    /// Get minimal Dimensions of the Widget.
    fn min_dimensions(&self) -> Dimensions<D>;
    /// Draw the widget.
    fn draw(&mut self, internal: &WidgetInternal<P, D>) -> bool;
    /// Update the status of the widget.
    fn update(&mut self, internal: &mut WidgetInternal<P, D>);
    /// Update the layout of the widget.
    fn layout(&mut self, internal: &mut WidgetInternal<P, D>);
    /// Containers search for widgets that are members of the same signal and then 
    /// call this function on found widgets.
    fn handle_signal(&mut self, internal: &mut WidgetInternal<P, D>, signal: SignalID);
    /// Handle a mouse state, Containers check if the mouse is on area or is grabbed and then
    /// call this function.
    fn handle_mouse(&mut self, internal: &mut WidgetInternal<P, D>, mouse: &MouseState<P>);
    /// Handle a keyboard state, it only be called if the widget is focused by a Container.
    fn handle_keys(&mut self, internal: &mut WidgetInternal<P, D>, key: &KeyState);
    /// Containers call this function for check if the widget should be focused or not by stepping.
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

/// General Widget Data, this is injected as arguments on widget trait functions
pub struct WidgetInternal<P, D> {
    /// Dimensions
    dim: Dimensions<D>,
    /// Minimun dimensions
    min_dim: Dimensions<D>,
    /// Relative position
    rel_pos: Position<P>,
    /// Absolute position
    abs_pos: Position<P>,
    /// Every Widget Flags
    flags: Flags,
    /// Event ID
    signal: Signal,
}

impl<P, D> WidgetInternal<P, D> {
    // FLAGS
    /// Set on or off to requested flags
    pub fn set(&mut self, flag: Flags, toggle: bool) {
        if toggle {
            self.flags |= flag | CHANGED;
        } else {
            self.flags = self.flags & !flag | CHANGED;
        }
    }

    #[inline]
    /// Toggle requested flags
    pub fn toggle(&mut self, flag: Flags) {
        self.flags ^= flag;
    }

    #[inline]
    /// Turn on requested flags
    pub fn on(&mut self, flag: Flags) {
        self.flags |= flag | CHANGED;
    }

    #[inline]
    /// Turn off requested flags
    pub fn off(&mut self, flag: Flags) {
        self.flags = self.flags & !flag | CHANGED;
    }

    #[inline]
    /// Check if at least one of the requested flags is enabled
    pub fn check_any(&self, flag: Flags) -> bool {
        flag & self.flags > 0
    }

    #[inline]
    /// Check if the requested flags are enabled
    pub fn check(&self, flag: Flags) -> bool {
        flag & self.flags == flag
    }

    #[inline]
    /// Check if the flags are changed
    pub fn changed(&mut self) -> bool {
        let ch = self.flags & CHANGED != 0;
        self.flags &= !CHANGED;

        ch
    }

    /// Hide the changes of the flags
    #[inline]
    pub fn unchange(&mut self) {
        self.flags &= !CHANGED;
    }

    #[inline]
    /// Get numeric value of the requested flags
    pub fn val(&self, flag: Flags) -> Flags {
        flag & self.flags
    }

    #[inline]
    /// Get numeric value of the flags
    pub fn flags(&self) -> Flags {
        self.flags
    }

    #[inline]
    /// Get a reference of the signal
    pub fn signal(&self) -> &Signal {
        &self.signal
    }

    #[inline]
    /// Get a mutable reference of the signal
    pub fn signal_mut(&mut self) -> &mut Signal {
        &mut self.signal
    }

    /// Changes the signal
    pub fn set_signal(&mut self, signal: Signal) {
        self.signal = signal;
    }
}

impl<P: Sized + Copy + Clone, D: Sized + Copy + Clone> WidgetInternal<P, D>
where
    D: PartialOrd + Default,
    P: Add<Output = P> + PartialOrd + From<D> + Default,
{
    /// Create a new Internal with Flags and Signal, all boundaries are initialized with 0
    pub fn new(flags: Flags, signal: Signal) -> Self {
        WidgetInternal {
            dim: (Default::default(), Default::default()),
            min_dim: (Default::default(), Default::default()),
            rel_pos: (Default::default(), Default::default()),
            abs_pos: (Default::default(), Default::default()),
            flags,
            signal,
        }
    }

    /// Create a new Internal with Flags, Signal, Relative Position and Dimensions
    pub fn new_with(
        rel_pos: Position<P>,
        dim: Dimensions<D>,
        flags: Flags,
        signal: Signal,
    ) -> Self {
        WidgetInternal {
            dim,
            min_dim: (Default::default(), Default::default()),
            rel_pos,
            abs_pos: (Default::default(), Default::default()),
            flags,
            signal,
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

    /// Set relative position and dimensions with boundaries tuple
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

    /// Sum other relative with self relative
    pub fn calc_absolute(&mut self, rel_pos: Position<P>) {
        self.abs_pos = (rel_pos.0 + self.rel_pos.0, rel_pos.1 + self.rel_pos.1);
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

    /// Change x relative coordinate
    pub fn set_x(&mut self, x: P) {
        self.rel_pos.0 = x;
    }

    /// Change y relative coordinate
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

    /// Get Boundaries with relative position
    #[inline]
    pub fn boundaries_rel(&self) -> Boundaries<P, D> {
        (self.rel_pos.0, self.rel_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get Boundaries with absolute position
    pub fn boundaries_abs(&self) -> Boundaries<P, D> {
        (self.abs_pos.0, self.abs_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get relative Position
    pub fn relative_pos(&self) -> Position<P> {
        self.rel_pos
    }

    /// Get absolute Position
    #[inline]
    pub fn absolute_pos(&self) -> Position<P> {
        self.abs_pos
    }

    /// Get minimum dimensions, useful for layouts
    #[inline]
    pub fn min_dimensions(&self) -> Dimensions<D> {
        self.min_dim
    }

    #[inline]
    /// Get dimensions
    pub fn dimensions(&self) -> Dimensions<D> {
        self.dim
    }

    /// Check if a point/cursor is on widget area. It checks using absolute position
    #[inline]
    pub fn on_area(&self, cursor: Position<P>) -> bool {
        self.check(VISIBLE)
            && cursor.0 >= self.abs_pos.0
            && cursor.0 <= self.abs_pos.0 + P::from(self.dim.0)
            && cursor.1 >= self.abs_pos.1
            && cursor.1 <= self.abs_pos.1 + P::from(self.dim.1)
    }
}
