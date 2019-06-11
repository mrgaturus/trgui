//! Structs and Traits for widgets

use crate::group::{Group, GroupID};
use crate::state::{KeyState, MouseState};
use std::ops::Add;

pub type Position<T> = (T, T);
pub type Dimensions<T> = (T, T);
pub type Boundaries<T> = (T, T, T, T);

// BITFLAGS (Sorry for no use the crate)
pub type Flags = u16;

pub mod flags {
    use crate::widget::Flags;

    pub const SIGNAL: Flags = 0b00000001;
    pub const DRAW: Flags = 0b00000010;
    pub const UPDATE: Flags = 0b00000100;
    pub const VISIBLE: Flags = 0b00001000;
    pub const ENABLED: Flags = 0b00010000;
    pub const HOVER: Flags = 0b00100000;
    pub const GRAB: Flags = 0b01000000;
    pub const FOCUS: Flags = 0b10000000;
    pub const LAYOUT: Flags = 0b100000000;
}

use flags::{DRAW, FOCUS, VISIBLE};

/// Main Widget Trait
pub trait Widget<T: Sized + Copy + Clone>
where
    T: Add<Output = T> + PartialOrd + Default,
{
    /// Get minimal Dimensions of the Widget.
    fn min_dimensions(&self) -> Dimensions<T> {
        (Default::default(), Default::default())
    }
    /// Draw the widget.
    fn draw(&mut self, _: &WidgetInternal<T>) -> bool {
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
    fn handle_keys(&mut self, _: &mut WidgetInternal<T>, _: &KeyState) {}
    /// Containers call this function for check if the widget should be focused or not by stepping.
    fn step_focus(&mut self, internal: &mut WidgetInternal<T>, _: bool) -> bool {
        let check = !internal.check(FOCUS);
        internal.on(DRAW);

        check
    }
    /// When you unhover the widget
    fn unhover(&mut self, internal: &mut WidgetInternal<T>) {
        internal.on(DRAW);
    }
    /// When you unfocus the widget
    fn unfocus(&mut self, internal: &mut WidgetInternal<T>) {
        internal.on(DRAW);
    }
}

/// General Widget Data, this is injected as arguments on widget trait functions
pub struct WidgetInternal<T> {
    /// Dimensions
    dim: Dimensions<T>,
    /// Minimun dimensions
    min_dim: Dimensions<T>,
    /// Relative position
    rel_pos: Position<T>,
    /// Absolute position
    abs_pos: Position<T>,
    /// Every Widget Flags
    flags: Flags,
    /// Event ID
    group: Group,
}

impl<T> WidgetInternal<T> {
    // FLAGS
    /// Set on or off to requested flags
    pub fn set(&mut self, flag: Flags, toggle: bool) {
        if toggle {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    #[inline]
    /// Replace all flags by others
    pub fn replace_all(&mut self, flags: Flags) {
        self.flags = flags;
    }

    #[inline]
    /// Replace flags by turn off a pattern and turn on other flags
    pub fn replace(&mut self, mask: Flags, values: Flags) {
        self.flags &= !mask | mask & values
    }

    #[inline]
    /// Toggle requested flags
    pub fn toggle(&mut self, flag: Flags) {
        self.flags ^= flag;
    }

    #[inline]
    /// Turn on requested flags
    pub fn on(&mut self, flag: Flags) {
        self.flags |= flag;
    }

    #[inline]
    /// Turn off requested flags
    pub fn off(&mut self, flag: Flags) {
        self.flags &= !flag;
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
    T: Add<Output = T> + PartialOrd + Default,
{
    /// Create a new Internal with Flags and Group, all boundaries are initialized with 0
    pub fn new(flags: Flags, group: Group) -> Self {
        WidgetInternal {
            dim: (Default::default(), Default::default()),
            min_dim: (Default::default(), Default::default()),
            rel_pos: (Default::default(), Default::default()),
            abs_pos: (Default::default(), Default::default()),
            flags,
            group,
        }
    }

    /// Create a new Internal with Flags, Group, Relative Position and Dimensions
    pub fn new_with(rel_pos: Position<T>, dim: Dimensions<T>, flags: Flags, group: Group) -> Self {
        WidgetInternal {
            dim,
            min_dim: (Default::default(), Default::default()),
            rel_pos,
            abs_pos: (Default::default(), Default::default()),
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
        self.rel_pos = (bounds.0, bounds.1);
        self.dim = (bounds.2, bounds.3);

        self.check_min();
    }

    /// Change position
    pub fn set_position(&mut self, pos: Position<T>) {
        self.rel_pos.0 = pos.0;
        self.rel_pos.1 = pos.1;
    }

    /// Sum other relative with self relative
    pub fn calc_absolute(&mut self, rel_pos: Position<T>) {
        self.abs_pos = (rel_pos.0 + self.rel_pos.0, rel_pos.1 + self.rel_pos.1);
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
        self.rel_pos.0 = x;
    }

    /// Change y relative coordinate
    pub fn set_y(&mut self, y: T) {
        self.rel_pos.1 = y;
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
        (self.rel_pos.0, self.rel_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get Boundaries with absolute position
    pub fn boundaries_abs(&self) -> Boundaries<T> {
        (self.abs_pos.0, self.abs_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get relative Position
    pub fn relative_pos(&self) -> Position<T> {
        self.rel_pos
    }

    /// Get absolute Position
    #[inline]
    pub fn absolute_pos(&self) -> Position<T> {
        self.abs_pos
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
    pub fn on_area(&self, cursor: Position<T>) -> bool {
        self.check(VISIBLE)
            && cursor.0 >= self.abs_pos.0
            && cursor.0 <= self.abs_pos.0 + self.dim.0
            && cursor.1 >= self.abs_pos.1
            && cursor.1 <= self.abs_pos.1 + self.dim.1
    }
}
