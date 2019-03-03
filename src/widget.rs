use crate::state::{MouseState, KeyState};

pub type Boundaries = (i32, i32, i32, i32);
pub type Dimensions = (i32, i32);

// BITFLAGS (Sorry for no use the crate)
const CHANGED: u8 =     0b00000001;
pub mod flags {
    pub const DRAW: u8 =    0b00000010;
    pub const UPDATE: u8 =  0b00000100;
    pub const VISIBLE: u8 = 0b00001000;
    pub const ENABLED: u8 = 0b00010000;
    pub const HOVER: u8 =   0b00100000;
    pub const GRAB: u8 =    0b01000000;
    pub const FOCUS: u8 =   0b10000000;
}

// TODO: create a check_bind

/// A Widget trait is used for the general methods that can be used on every widget.
pub trait Widget {
    /// Get minimal Dimensions of the Widget
    fn compute_min(&self) -> Dimensions { (0, 0) }
    /// Draw the widget
    fn draw(&mut self, internal: &WidgetInternal) -> bool;
    /// Update the status of the widget
    fn update(&mut self, internal: &WidgetInternal) -> bool;
    /// Update the layout of the widget
    fn update_layout(&mut self, internal: &mut WidgetInternal);
    /// Handle a mouse state (focus, grab)
    fn handle_mouse(&mut self, mouse: &MouseState, internal: &mut WidgetInternal);
    /// Handle a keyboard state
    fn handle_keys(&mut self, key: &KeyState, internal: &mut WidgetInternal);
    /// Step the focus
    fn step_focus(&mut self, _: bool, internal: &mut WidgetInternal) -> bool {
        let focus = !internal.check(flags::FOCUS | flags::ENABLED);
        if focus {
            internal.on(flags::DRAW);
        }
        
        focus
    }
    /// When you unhover the widget
    fn unhover(&mut self, internal: &mut WidgetInternal) { internal.on(flags::DRAW); }
    /// When you unfocus the widget
    fn unfocus(&mut self, internal: &mut WidgetInternal) { internal.on(flags::DRAW); }

    // Move the widget to the heap
    #[inline]
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}

pub struct WidgetInternal {
    /// Cordinates and Dimensions as a tuple (x, y, width, height)
    bounds: Boundaries,
    /// Minimun dimensions
    min_dim: Dimensions,
    /// Absolute position
    abs_pos: Dimensions,
    /// Every Widget Flag
    flags: u8
}

impl WidgetInternal {
    pub fn new(bounds: Boundaries, flags: u8) -> Self {
        WidgetInternal {
            bounds,
            min_dim: (0, 0),
            abs_pos: (0, 0),
            flags
        }
    }

    // FLAGS
    
    pub fn set(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= flag | CHANGED;
        } else {
            self.flags = self.flags & !flag | CHANGED;
        }
    }

    #[inline]
    pub fn replace(&mut self, flags: u8) {
        self.flags = self.flags & !flags | flags | CHANGED;
    }

    #[inline]
    pub fn toggle(&mut self, flag: u8) {
        self.flags ^= flag;
    }

    #[inline]
    pub fn on(&mut self, flag: u8) {
        self.flags |= flag | CHANGED;
    }

    #[inline]
    pub fn off(&mut self, flag: u8) {
        self.flags = self.flags & !flag | CHANGED;
    }

    #[inline]
    pub fn check_any(&self, flag: u8) -> bool {
        flag & self.flags > 0
    }

    #[inline]
    pub fn check(&self, flag: u8) -> bool {
        flag & self.flags == flag
    }

    #[inline]
    pub fn changed(&mut self) -> bool {
        let ch = self.flags & CHANGED != 0;
        self.flags &= !CHANGED;
        
        ch
    }

    #[inline]
    pub fn val(&self, flag: u8) -> u8 {
        flag & self.flags
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
        self.abs_pos = absolute_pos!(pos, self.bounds);
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
}