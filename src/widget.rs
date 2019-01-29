use crate::state::{MouseState, KeyState};

pub type Boundaries = (i32, i32, i32, i32);
pub type Dimensions = (i32, i32);

/// A Widget trait is used for the general methods that can be used on every widget.
pub trait Widget {
    /// Draw the current widget
    fn draw(&self, position: &(i32, i32), internal: &WidgetInternal);
    /// Update the status of widget
    fn update(&mut self, layout: bool, internal: &mut WidgetInternal);
    /// Handle a mouse state (focus, grab)
    fn handle_mouse(&mut self, mouse: &MouseState, internal: &mut WidgetInternal);
    /// Handle a keyboard state
    fn handle_keys(&mut self, key: &KeyState);
    /// Step the focus
    fn step_focus(&mut self, back: bool, internal: &mut WidgetInternal) -> bool;
    /// Get minimal Dimensions of the Widget
    fn get_min(&self) -> Dimensions;
    /// When you unhover the widget
    fn unhover(&mut self);
    /// When you unfocus the widget
    fn unfocus(&mut self);


    // Move the widget to the heap
    #[inline]
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}

/// WidgetInternal holds all boundary and coordinate information of the widget. This is used at composition
/// and can be optional.
pub struct WidgetInternal {
    /// Cordinates and Boundaries as a tuple (x, y, width, height)
    bounds: Boundaries,
    /// Focus handling
    focus: bool,
    /// Grab
    grab: bool,
    /// Hover
    hover: bool,
    /// Enabled
    enable: bool,
    /// Visible (widget, layout)
    show: (bool, bool),
    /// Needs update?
    update: bool
}

impl WidgetInternal {
    /// Create a new widget internal with coordinates and boundaries tuples
    pub fn new(bounds: Boundaries, update: bool) -> Self {
        WidgetInternal {
            bounds,
            focus: false,
            grab: false,
            hover: false,
            enable: true,
            show: (true, true),
            update
        }
    }

    /// Change boundaries
    pub fn set_boundaries(&mut self, bounds: Boundaries) {
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
    pub fn boundaries(&self) -> Boundaries {
        self.bounds
    }

    /// Get coordinates tuple
    pub fn coordinates(&self) -> Dimensions {
        (self.bounds.0, self.bounds.1)
    }

    /// Get dimensions tuple
    pub fn dimensions(&self) -> Dimensions {
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
        self.show.0 = visible;
    }

    pub fn set_visible_layout(&mut self, visible: bool) {
        self.show.1 = visible;
    }

    /// Set if is grabbed
    pub fn set_grab(&mut self, grab: bool) {
        self.grab = grab;
    }

    /// Set if is enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enable = enabled;
    }

    /// Set if is enabled
    pub fn set_hover(&mut self, hover: bool) {
        self.hover = hover;
    }

    /// Set if needs update
    pub fn set_update(&mut self, update: bool) {
        self.update = update;
    }

    /// Toggle enabled
    pub fn toggle_enabled(&mut self) {
        self.enable = !self.enable;
    }

    /// Toggle hovered
    pub fn toggle_hovered(&mut self) {
        self.hover = !self.hover;
    }

    /// Toggle focused
    pub fn toggle_focus(&mut self) {
        self.focus = !self.focus;
    }

    /// Toggle update
    pub fn toggle_update(&mut self) {
        self.update = !self.update;
    }

    /// Check if is enabled
    pub fn enabled(&self) -> bool {
        self.enable
    }

    /// Check if is focused
    pub fn focused(&self) -> bool {
        self.focus
    }

    /// Check if is hovered
    pub fn hovered(&self) -> bool {
        self.hover
    }

    /// Check if is visible
    pub fn visible(&self) -> bool {
        self.show.0 && self.show.1
    }

    /// Check if is grabbed
    pub fn grabbed(&self) -> bool {
        self.grab
    }

    /// Check if needs update
    pub fn need_update(&self) -> bool {
        self.update
    }
}