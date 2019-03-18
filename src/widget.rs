use crate::state::{MouseState, KeyState};

pub type Boundaries = (i32, i32, i32, i32);
pub type Dimensions = (i32, i32);

// BITFLAGS (Sorry for no use the crate)
type FlagType = u16;

const CHANGED: FlagType =     0b00000001;
pub mod flags {
    use crate::widget::FlagType;

    pub const DRAW: FlagType =    0b00000010;
    pub const UPDATE: FlagType =  0b00000100;
    pub const VISIBLE: FlagType = 0b00001000;
    pub const ENABLED: FlagType = 0b00010000;
    pub const HOVER: FlagType =   0b00100000;
    pub const GRAB: FlagType =    0b01000000;
    pub const FOCUS: FlagType =   0b10000000;

    pub const UPDATE_BIND: FlagType = 0b100000000;
}

use flags::{FOCUS, VISIBLE, DRAW};

// TODO: create a check_bind

/// A Widget trait is used for the general methods that can be used on every widget.
pub trait Widget {
    /// Get minimal Dimensions of the Widget
    fn compute_min(&self) -> Dimensions { (0, 0) }
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
    /// Cordinates and Dimensions as a tuple (x, y, width, height)
    bounds: Boundaries,
    /// Minimun dimensions
    min_dim: Dimensions,
    /// Absolute position
    abs_pos: Dimensions,
    /// Every Widget Flag
    flags: FlagType
}

impl WidgetInternal {
    pub fn new(bounds: Boundaries, flags: FlagType) -> Self {
        WidgetInternal {
            bounds,
            min_dim: (0, 0),
            abs_pos: (0, 0),
            flags
        }
    }

    // FLAGS

    pub fn set(&mut self, flag: FlagType, value: bool) {
        if value {
            self.flags |= flag | CHANGED;
        } else {
            self.flags = self.flags & !flag | CHANGED;
        }
    }

    #[inline]
    pub fn toggle(&mut self, flag: FlagType) {
        self.flags ^= flag;
    }

    #[inline]
    pub fn on(&mut self, flag: FlagType) {
        self.flags |= flag | CHANGED;
    }

    #[inline]
    pub fn off(&mut self, flag: FlagType) {
        self.flags = self.flags & !flag | CHANGED;
    }

    #[inline]
    pub fn check_any(&self, flag: FlagType) -> bool {
        flag & self.flags > 0
    }

    #[inline]
    pub fn check(&self, flag: FlagType) -> bool {
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
    pub fn val(&self, flag: FlagType) -> FlagType {
        flag & self.flags
    }

    #[inline]
    pub fn val_all(&self) -> FlagType {
        self.flags
    }

    fn check_min(&mut self) {
        if self.bounds.2 < self.min_dim.0 {
            self.bounds.2 = self.min_dim.0;
        }
        if self.bounds.3 < self.min_dim.1 {
            self.bounds.3 = self.min_dim.1;
        }
    }

    // BOUNDARIES

    /// Change boundaries
    pub fn set_boundaries(&mut self, bounds: Boundaries) {
        self.bounds = bounds;

        self.check_min();
    }

    /// Change coordinates
    pub fn set_coordinates(&mut self, dim: Dimensions) {
        self.bounds.0 = dim.0;
        self.bounds.1 = dim.1;
    }

    /// Sum absolute position
    pub fn compute_absolute(&mut self, pos: Dimensions) {
        let coords = self.coordinates();

        self.abs_pos = (pos.0 + coords.0, pos.1 + coords.1);
    }

    /// Change dimensions
    pub fn set_dimensions(&mut self, dim: Dimensions) {
        self.bounds.2 = dim.0;
        self.bounds.3 = dim.1;

        self.check_min();
    }

    /// Change minimal dimensions
    pub fn set_min_dimensions(&mut self, dim: Dimensions) {
        self.min_dim = dim;

        self.check_min();
    }

    /// Change x coordinate
    pub fn set_x(&mut self, x: i32) {
        self.bounds.0 = x;
    }

    /// Change y coordinate
    pub fn set_y(&mut self, y: i32) {
        self.bounds.1 = y;
    }

    /// Change width
    pub fn set_width(&mut self, width: i32) {
        self.bounds.2 = width;

        self.check_min();
    }

    /// Change height
    pub fn set_height(&mut self, height: i32) {
        self.bounds.3 = height;

        self.check_min();
    }

    /// Get all boundaries
    #[inline]
    pub fn boundaries(&self) -> Boundaries {
        self.bounds
    }

    /// Get All Boundaries with absolute coordinates
    pub fn boundaries_abs(&self) -> Boundaries {
        (self.abs_pos.0, self.abs_pos.1, self.bounds.2, self.bounds.3)
    }

    /// Get coordinates tuple
    pub fn coordinates(&self) -> Dimensions {
        (self.bounds.0, self.bounds.1)
    }

    /// Get dimensions tuple
    pub fn dimensions(&self) -> Dimensions {
        (self.bounds.2, self.bounds.3)
    }

    pub fn min_dimensions(&self) -> Dimensions {
        self.min_dim
    }

    pub fn absolute_pos(&self) -> Dimensions {
        self.abs_pos
    }

    #[inline]
    pub fn on_area(&self, cursor: Dimensions) -> bool {
        let bounds = self.boundaries_abs();

        self.check(VISIBLE) && cursor.0 >= bounds.0 && cursor.0 <= bounds.0 + bounds.2 &&
        cursor.1 >= bounds.1 && cursor.1 <= bounds.1 + bounds.3
    }
}