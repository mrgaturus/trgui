//! Structs and Traits for widgets

use crate::group::{Group, GroupID};
use crate::state::{KeyState, MouseState};
use std::ops::{Add, Sub};

pub type Position<T> = (T, T);
pub type Dimensions<T> = (T, T);
pub type Boundaries<T> = (T, T, T, T);

// BITFLAGS (Sorry for no use the crate)
pub mod flags {
    pub type Flags = u16;
    /// Default flags for be added on containers ( ENABLED | VISIBLE )
    pub const WIDGET: Flags = 0b00011000;
    /// Default flags for be added on containers, with signal handling enabled
    /// ( ENABLED | VISIBLE | SIGNAL )
    pub const WIDGET_S: Flags = 0b00011001;

    pub const SIGNAL: Flags = 0b00000001;
    pub const DRAW: Flags = 0b00000010;
    pub const UPDATE: Flags = 0b00000100;
    pub const VISIBLE: Flags = 0b00001000;
    pub const ENABLED: Flags = 0b00010000;
    pub const HOVER: Flags = 0b00100000;
    pub const GRAB: Flags = 0b01000000;
    pub const FOCUS: Flags = 0b10000000;
    pub const LAYOUT: Flags = 0b00000001_00000000;
    pub const PREV_LAYOUT: Flags = 0b00000010_00000000;
}

use flags::{Flags, DRAW, FOCUS, VISIBLE};

/// Main Widget Trait
pub trait Widget<T: Sized + Copy + Clone, CTX: Sized>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Default,
{
    /// Draw the widget.
    fn draw(&mut self, _: &WidgetInternal<T>, _: &mut CTX) -> bool {
        false
    }
    /// Update the status of the widget.
    fn update(&mut self, _: &mut WidgetInternal<T>) {}
    /// Update the layout of the widget.
    fn layout(&mut self, _: &mut WidgetInternal<T>, _: bool) {}
    /// Containers search for widgets that are members of the same Group and then
    /// call this function on found widgets.
    fn handle_signal(&mut self, _: &mut WidgetInternal<T>, _: GroupID) {}
    /// Handle a mouse state, Containers check if the mouse is on area or is grabbed and then
    /// call this function.
    fn handle_mouse(&mut self, _: &mut WidgetInternal<T>, _: &MouseState<T>) {}
    /// Handle a keyboard state, it only be called if the widget is focused by a Container.
    fn handle_keys(&mut self, _: &mut WidgetInternal<T>, _: KeyState) {}
    /// Containers call this function for check if the widget should be focused or not by stepping.
    fn step_focus(&mut self, internal: &mut WidgetInternal<T>, _: bool) -> bool {
        let check = !internal.check(FOCUS);
        internal.on(DRAW);

        check
    }
    /// When you unhover the widget
    fn hover_out(&mut self, internal: &mut WidgetInternal<T>) {
        internal.on(DRAW);
    }
    /// When you unfocus the widget
    fn focus_out(&mut self, internal: &mut WidgetInternal<T>) {
        internal.on(DRAW);
    }
    /// Get minimal Dimensions of the Widget.
    fn min_dimensions(&self) -> Dimensions<T> {
        (Default::default(), Default::default())
    }
}

/// General Widget Data, this is injected as arguments on widget trait functions
pub struct WidgetInternal<T> {
    /// Every Widget Flags
    pub flags: Flags,
    /// Dimensions
    dim: Dimensions<T>,
    /// Minimun dimensions
    min_dim: Dimensions<T>,
    /// Pivot position
    p_pos: Position<T>,
    /// Absolute position
    pos: Position<T>,
    /// Event ID
    group: Group,
}

impl<T> WidgetInternal<T> {
    // FLAGS
    /// Set on or off to requested flags
    pub fn turn(&mut self, mask: Flags, toggle: bool) {
        if toggle {
            self.flags |= mask;
        } else {
            self.flags &= !mask;
        }
    }

    #[inline]
    /// Replace flags by turn off a pattern and turn on other flags
    pub fn replace(&mut self, mask: Flags, flags: Flags) {
        self.flags &= !mask | mask & flags;
    }

    #[inline]
    /// Toggle requested flags
    pub fn toggle(&mut self, mask: Flags) {
        self.flags ^= mask;
    }

    #[inline]
    /// Turn on requested flags
    pub fn on(&mut self, mask: Flags) {
        self.flags |= mask;
    }

    #[inline]
    /// Turn off requested flags
    pub fn off(&mut self, mask: Flags) {
        self.flags &= !mask;
    }

    #[inline]
    /// Turn off flags and then turn on other flags
    pub fn off_on(&mut self, off_mask: Flags, on_mask: Flags) {
        self.flags = self.flags & !off_mask | on_mask;
    }

    /// Get numeric value of the requested flags and then off with other mask
    #[inline]
    pub fn drain(&mut self, mut get_mask: Flags, off_mask: Flags) -> Flags {
        get_mask = get_mask & self.flags;
        self.flags &= !off_mask;

        get_mask
    }

    #[inline]
    /// Check if at least one of the requested flags is enabled
    pub fn check_any(&self, mask: Flags) -> bool {
        mask & self.flags > 0
    }

    #[inline]
    /// Check if the requested flags are enabled
    pub fn check(&self, mask: Flags) -> bool {
        mask & self.flags == mask
    }

    #[inline]
    /// Get numeric value of the requested flags
    pub fn val(&self, mask: Flags) -> Flags {
        mask & self.flags
    }

    #[inline]
    /// Get a reference of the Group
    pub fn group(&self) -> &Group {
        &self.group
    }

    /// Changes the Group
    pub fn set_group(&mut self, group: Group) {
        self.group = group;
    }
}

impl<T: Sized + Copy + Clone> WidgetInternal<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Default,
{
    /// Create a new Internal with Flags and Group, all boundaries are initialized with 0
    pub fn new(flags: Flags, group: Group) -> Self {
        WidgetInternal {
            dim: (Default::default(), Default::default()),
            min_dim: (Default::default(), Default::default()),
            pos: (Default::default(), Default::default()),
            p_pos: (Default::default(), Default::default()),
            flags,
            group,
        }
    }

    /// Create a new Internal with Flags, Group, Relative Position and Dimensions
    pub fn new_with(pos: Position<T>, dim: Dimensions<T>, flags: Flags, group: Group) -> Self {
        WidgetInternal {
            dim,
            min_dim: (Default::default(), Default::default()),
            pos,
            p_pos: (Default::default(), Default::default()),
            flags,
            group,
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
    pub fn set_boundaries(&mut self, bounds: Boundaries<T>) {
        self.pos = (self.p_pos.0 + bounds.0, self.p_pos.1 + bounds.1);
        self.dim = (bounds.2, bounds.3);

        self.check_min();
    }

    /// Change position
    pub fn set_position(&mut self, pos: Position<T>) {
        self.pos = (self.p_pos.0 + pos.0, self.p_pos.1 + pos.1);
    }

    /// Change pivot
    pub fn set_pivot(&mut self, pivot: Position<T>) {
        self.pos = (
            pivot.0 + self.pos.0 - self.p_pos.0,
            pivot.1 + self.pos.1 - self.p_pos.1,
        );
        self.p_pos = pivot;
    }

    /// Change dimensions
    pub fn set_dimensions(&mut self, dim: Dimensions<T>) {
        self.dim = dim;

        self.check_min();
    }

    /// Change minimal dimensions
    pub fn set_min_dimensions(&mut self, dim: Dimensions<T>) {
        self.min_dim = dim;

        self.check_min();
    }

    /// Change x relative coordinate
    pub fn set_x(&mut self, x: T) {
        self.pos.0 = self.p_pos.0 + x;
    }

    /// Change y relative coordinate
    pub fn set_y(&mut self, y: T) {
        self.pos.1 = self.p_pos.1 + y;
    }

    /// Change width
    pub fn set_width(&mut self, width: T) {
        self.dim.0 = width;

        self.check_min();
    }

    /// Change height
    pub fn set_height(&mut self, height: T) {
        self.dim.1 = height;

        self.check_min();
    }

    /// Get Boundaries with relative position
    #[inline]
    pub fn boundaries_rel(&self) -> Boundaries<T> {
        (
            self.pos.0 - self.p_pos.0,
            self.pos.1 - self.p_pos.1,
            self.dim.0,
            self.dim.1,
        )
    }

    #[inline]
    /// Get Boundaries with absolute position
    pub fn boundaries_abs(&self) -> Boundaries<T> {
        (self.pos.0, self.pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get relative Position
    pub fn relative_pos(&self) -> Position<T> {
        (self.pos.0 - self.p_pos.0, self.pos.1 - self.p_pos.1)
    }

    /// Get absolute Position
    #[inline]
    pub fn absolute_pos(&self) -> Position<T> {
        self.pos
    }

    /// Get minimum dimensions, useful for layouts
    #[inline]
    pub fn min_dimensions(&self) -> Dimensions<T> {
        self.min_dim
    }

    #[inline]
    /// Get dimensions
    pub fn dimensions(&self) -> Dimensions<T> {
        self.dim
    }

    /// Check if a point/cursor is on widget area. It checks using absolute position
    #[inline]
    pub fn p_intersect(&self, p: Position<T>) -> bool {
        self.flags & VISIBLE == VISIBLE
            && p.0 >= self.pos.0
            && p.0 <= self.pos.0 + self.dim.0
            && p.1 >= self.pos.1
            && p.1 <= self.pos.1 + self.dim.1
    }
}