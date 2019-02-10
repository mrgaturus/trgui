use std::ops::BitOr;
use crate::state::{MouseState, KeyState};

pub type Boundaries = (i32, i32, i32, i32);
pub type Dimensions = (i32, i32);

/// A Widget trait is used for the general methods that can be used on every widget.
pub trait Widget {
    fn update(&mut self, internal: &mut WidgetInternal);
    /// Update the layout of the widget
    fn update_layout(&mut self, internal: &mut WidgetInternal);
    /// Handle a mouse state (focus, grab)
    fn handle_mouse(&mut self, mouse: &MouseState, internal: &mut WidgetInternal);
    /// Handle a keyboard state
    fn handle_keys(&mut self, key: &KeyState, internal: &mut WidgetInternal);
    /// Step the focus
    fn step_focus(&mut self, back: bool, internal: &mut WidgetInternal) -> bool;
    /// Get minimal Dimensions of the Widget
    fn compute_min(&self) -> Dimensions;
    /// When you unhover the widget
    fn unhover(&mut self, internal: &mut WidgetInternal);
    /// When you unfocus the widget
    fn unfocus(&mut self, internal: &mut WidgetInternal);


    // Move the widget to the heap
    #[inline]
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}

pub enum WidgetFlag {
    Changed,
    Draw,
    Update,
    Visible,
    Enabled,
    Hover,
    Grab,
    Focus
}

impl BitOr for WidgetFlag {
    type Output = u8;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> u8 {
        (1 << self as u8) | (1 << rhs as u8)
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
    pub fn set(&mut self, flag: WidgetFlag, value: bool) {
        let bit: u8 = flag as u8;
        let mask: u8 = 1 << bit;

        self.flags = self.flags & !mask | (value as u8) << bit & mask | 1;
    }

    #[inline]
    pub fn toggle(&mut self, flag: WidgetFlag) {
        self.flags ^= 1 << flag as u8;
    }

    #[inline]
    pub fn on(&mut self, flag: WidgetFlag) {
        self.flags |= (1 << flag as u8) | 1;
    }

    #[inline]
    pub fn off(&mut self, flag: WidgetFlag) {
        self.flags = self.flags & !(1 << flag as u8) | 1;
    }

    #[inline]
    pub fn get(&self, flag: WidgetFlag) -> bool {
        (1 << flag as u8) & self.flags != 0
    }

    #[inline]
    pub fn get_u8(&self, flag: u8) -> bool {
        flag & self.flags != 0
    }

    #[inline]
    pub fn changed(&self) -> bool {
        self.flags & 1 != 0
    }

    #[inline]
    pub fn can_point(&self) -> bool {
        0b00011000 & self.flags != 0
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