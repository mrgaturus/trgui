use crate::state::{MouseState, KeyState};

pub type Position<P> = (P, P);
pub type Dimensions<D> = (D, D);
pub type Boundaries<P, D> = (P, P, D, D);

// BITFLAGS (Sorry for no use the crate)
type Flag = u16;

const CHANGED: Flag =     0b00000001;
pub mod flags {
    use crate::widget::Flag;

    pub const DRAW: Flag =    0b00000010;
    pub const UPDATE: Flag =  0b00000100;
    pub const VISIBLE: Flag = 0b00001000;
    pub const ENABLED: Flag = 0b00010000;
    pub const HOVER: Flag =   0b00100000;
    pub const GRAB: Flag =    0b01000000;
    pub const FOCUS: Flag =   0b10000000;

    pub const UPDATE_BIND: Flag = 0b100000000;
}

use flags::{FOCUS, VISIBLE, DRAW};

// TODO: create a check_bind

/// A Widget trait is used for the general methods that can be used on every widget.
pub trait Widget {
    /// Get minimal (i32, i32) of the Widget
    fn compute_min(&self) -> (i32, i32) { (0, 0) }
    /// Draw the widget
    fn draw(&mut self, internal: &WidgetInternal) -> bool;
    /// Update the status of the widget
    fn update(&mut self, internal: &mut WidgetInternal, bind: bool);
    /// Update the layout of the widget
    fn update_layout(&mut self, internal: &mut WidgetInternal);
    /// Handle a mouse state (focus, grab)
    fn handle_mouse(&mut self, internal: &mut WidgetInternal, mouse: &MouseState);
    /// Handle a keyboard state
    fn handle_keys(&mut self, internal: &mut WidgetInternal, key: &KeyState);
    /// Step the focus
    fn step_focus(&mut self, internal: &mut WidgetInternal, _: bool) -> bool {
        let check = !internal.check(FOCUS);
        internal.set(DRAW, check && internal.check(VISIBLE));
        
        check
    }
    /// When you unhover the widget
    fn unhover(&mut self, internal: &mut WidgetInternal) { internal.on(DRAW); }
    /// When you unfocus the widget
    fn unfocus(&mut self, internal: &mut WidgetInternal) { internal.on(DRAW); }
}

pub struct WidgetInternal {
    /// (i32, i32)
    dim: (i32, i32),
    /// Minimun dimensions
    min_dim: (i32, i32),
    /// Relative position
    rel_pos: (i32, i32),
    /// Absolute position
    abs_pos: (i32, i32),
    /// Every Widget Flag
    flags: Flag
}

impl WidgetInternal {
    pub fn new(rel_pos: (i32, i32), dim: (i32, i32), flags: Flag) -> Self {
        WidgetInternal {
            dim,
            min_dim: (0, 0),
            rel_pos,
            abs_pos: (0, 0),
            flags
        }
    }

    // FLAGS

    pub fn set(&mut self, flag: Flag, value: bool) {
        if value {
            self.flags |= flag | CHANGED;
        } else {
            self.flags = self.flags & !flag | CHANGED;
        }
    }

    #[inline]
    pub fn toggle(&mut self, flag: Flag) {
        self.flags ^= flag;
    }

    #[inline]
    pub fn on(&mut self, flag: Flag) {
        self.flags |= flag | CHANGED;
    }

    #[inline]
    pub fn off(&mut self, flag: Flag) {
        self.flags = self.flags & !flag | CHANGED;
    }

    #[inline]
    pub fn check_any(&self, flag: Flag) -> bool {
        flag & self.flags > 0
    }

    #[inline]
    pub fn check(&self, flag: Flag) -> bool {
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
    pub fn val(&self, flag: Flag) -> Flag {
        flag & self.flags
    }

    #[inline]
    pub fn val_all(&self) -> Flag {
        self.flags
    }



    // BOUNDARIES

    fn check_min(&mut self) {
        if self.dim.0 < self.min_dim.0 {
            self.dim.0 = self.min_dim.0;
        }
        if self.dim.1 < self.min_dim.1 {
            self.dim.1 = self.min_dim.1;
        }
    }

    /// Set relative position and dimensions with 4 item tuple (x, y, width, height)
    pub fn set_boundaries(&mut self, bounds: (i32, i32, i32, i32)) {
        self.rel_pos = (bounds.0, bounds.1);
        self.dim = (bounds.2, bounds.3);

        self.check_min();
    }

    /// Change position
    pub fn set_position(&mut self, pos: (i32, i32)) {
        self.rel_pos.0 = pos.0;
        self.rel_pos.1 = pos.1;
    }

    /// Sum absolute position
    pub fn compute_absolute(&mut self, pos: (i32, i32)) {
        self.abs_pos = (pos.0 + self.rel_pos.0, pos.1 + self.rel_pos.1);
    }

    /// Change dimensions
    pub fn set_dimensions(&mut self, dim: (i32, i32)) {
        self.dim.0 = dim.0;
        self.dim.1 = dim.1;

        self.check_min();
    }

    /// Change minimal dimensions
    pub fn set_min_dimensions(&mut self, dim: (i32, i32)) {
        self.min_dim = dim;

        self.check_min();
    }

    /// Change x coordinate
    pub fn set_x(&mut self, x: i32) {
        self.rel_pos.0 = x;
    }

    /// Change y coordinate
    pub fn set_y(&mut self, y: i32) {
        self.rel_pos.1 = y;
    }

    /// Change width
    pub fn set_width(&mut self, width: i32) {
        self.dim.0 = width;

        self.check_min();
    }

    /// Change height
    pub fn set_height(&mut self, height: i32) {
        self.dim.1 = height;

        self.check_min();
    }

    /// Get (i32, i32, i32, i32) with relative position
    #[inline]
    pub fn boundaries_rel(&self) -> (i32, i32, i32, i32) {
        (self.rel_pos.0, self.rel_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get (i32, i32, i32, i32) with absolute position
    pub fn boundaries_abs(&self) -> (i32, i32, i32, i32) {
        (self.abs_pos.0, self.abs_pos.1, self.dim.0, self.dim.1)
    }

    #[inline]
    /// Get coordinates tuple
    pub fn relative_pos(&self) -> (i32, i32) {
        self.rel_pos
    }

    #[inline]
    pub fn absolute_pos(&self) -> (i32, i32) {
        self.abs_pos
    }

    #[inline]
    pub fn min_dimensions(&self) -> (i32, i32) {
        self.min_dim
    }

    #[inline]
    /// Get dimensions tuple
    pub fn dimensions(&self) -> (i32, i32) {
        self.dim
    }

    #[inline]
    pub fn on_area(&self, cursor: (i32, i32)) -> bool {
        self.check(VISIBLE) && 
        cursor.0 >= self.abs_pos.0 && 
        cursor.0 <= self.abs_pos.0 + self.dim.0 &&
        cursor.1 >= self.abs_pos.1 && 
        cursor.1 <= self.abs_pos.1 + self.dim.1
    }
}