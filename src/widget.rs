use crate::state::{MouseState, KeyState};
use crate::window::Window;

/// A Widget trait is used for the general methods that can be used on every widget.
pub trait Widget {
    /// Draw the current widget
    fn draw(&self);
    /// Update the status of widget
    fn update(&mut self);
    /// Handle an event state
    fn handle(&mut self, mouse: &MouseState, key: &KeyState);
    /// Get Widget Bounds (x, y, width, height)
    fn get_bounds(&self) -> (i32, i32, i32, i32);
    /// Set Widget Bounds (x, y, width, height)
    fn set_bounds(&mut self, bounds: (i32, i32, i32, i32));
}

/// WidgetGrab - Needs more commenting on what it actually does!
pub trait WidgetGrab: Widget {
    /// Grab for a window state
    unsafe fn grab(&mut self, window: &mut Window);
    /// Ungrab from a window state
    unsafe fn ungrab(&mut self, window: &mut Window);
}

/// WidgetInternal holds all boundary and coordinate information of the widget. This is used at composition
/// and can be optional.
pub struct WidgetInternal {
    /// Cordinates and Boundaries as a tuple (x, y, width, height)
    bounds: (i32, i32, i32, i32),
    /// Focus handling
    focus: bool,
    /// Grab
    grab: bool,
    /// Enabled
    enable: bool,
    /// Visible
    show: bool
}

impl WidgetInternal {
    /// Create a new widget internal with coordinates and boundaries tuples
    pub fn new(bounds: (i32, i32, i32, i32)) -> Self {
        WidgetInternal {
            bounds,
            focus: false,
            grab: false,
            enable: true,
            show: true
        }
    }

    /// Change boundaries
    pub fn set_boundaries(&mut self, bounds: (i32, i32, i32, i32)) {
        self.bounds = bounds;
    }

    /// Change coordinates
    pub fn set_coords(&mut self, x: i32, y: i32) {
        self.bounds.0 = x;
        self.bounds.1 = y;
    }

    /// Change dimensions
    pub fn set_dimensions(&mut self, width: i32, height: i32) {
        self.bounds.2 = width;
        self.bounds.3 = height;
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
    }

    /// Change height
    pub fn set_height(&mut self, height: i32) {
        self.bounds.3 = height;
    }

    /// Get all boundaries
    pub fn boundaries(&self) -> (i32, i32, i32, i32) {
        self.bounds
    }

    /// Get coordinates tuple
    pub fn coordinates(&self) -> (i32, i32) {
        (self.bounds.0, self.bounds.1)
    }

    /// Get boundaries tuple
    pub fn dimensions(&self) -> (i32, i32) {
        (self.bounds.2, self.bounds.3)
    }

    /// Get x coordinate
    pub fn x(&self) -> i32 {
        self.bounds.0
    }

    /// Get y coordinate
    pub fn y(&self) -> i32 {
        self.bounds.1
    }

    /// Get width
    pub fn width(&self) -> i32 {
        self.bounds.2
    }

    /// Get height
    pub fn height(&self) -> i32 {
        self.bounds.3
    }

    /// Set if is focused
    pub fn set_focused(&mut self, focus: bool) {
        self.focus = focus;
    }

    /// Set if is visible
    pub fn set_visible(&mut self, visible: bool) {
        self.show = visible;
    }

    /// Set if is grabbed
    pub fn set_grab(&mut self, grab: bool) {
        self.grab = grab;
    }

    /// Set if is enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enable = enabled;
    }

    /// Toggle if is enabled
    pub fn toggle_enabled(&mut self) {
        self.enable = !self.enable;
    }

    /// Check if is enabled
    pub fn enabled(&self) -> bool {
        self.enable
    }

    /// Check if is focused
    pub fn focused(&self) -> bool {
        self.focus
    }

    /// Check if is visible
    pub fn visible(&self) -> bool {
        self.show
    }

    /// Check if is grabbed
    pub fn grabbed(&self) -> bool {
        self.grab
    }
}