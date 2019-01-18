use crate::widget::{Widget, WidgetInternal, Boundaries, WidgetGrab};
use crate::state::{KeyState, MouseState};

pub struct Button {
    label: String,
    internal: WidgetInternal,
    clicked: bool
}

impl Button {
    pub fn new(label: &str, bounds: Boundaries) -> Self {
        Button {
            label: String::from(label),
            internal: WidgetInternal::new(bounds),
            clicked: false
        }
    }

    /// ------ UNTESTED ----------
    /// Get the current label of this widget.
    pub fn get_label(self) {
        self.label;
    }

    /// Set a new label for this widget.
    pub fn set_label(&mut self, label : String) {
        self.label = label;
    }

}

impl Widget for Button {
    /// Draw the current widget
    fn draw(&self) {
        //println!("BUTTON: ({}) ({:?})", self.label, self.get_bounds());
    }
    /// Update the status of widget
    fn update(&mut self) {

    }
    /// Handle an event state
    /// Handle a mouse state
    fn handle_mouse(&mut self, mouse: &MouseState) -> (bool, bool) {
        if mouse.clicked() {
            //println!("{} {} {:?} {:?}", "Clicked", self.label, mouse.coordinates_relative(), mouse.coordinates());
            self.clicked = true;
            return (false, self.grab());
        } else if self.clicked {
            self.clicked = false;
            return (self.focus(), self.ungrab());
        } else {
            //println!("{} {} {:?} {:?}", "Hovered", self.label, mouse.coordinates_relative(), mouse.coordinates());
        }

        (false, false)
    }
    /// Handle a keyboard state
    fn handle_keys(&mut self, _key: &KeyState) {
        
    }

    fn get_bounds(&self) -> Boundaries {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: Boundaries) {
        self.internal.set_boundaries(bounds);
    }

    /// Focus the current widget
    fn focus(&mut self) -> bool {
        self.internal.set_focused(true);
        true
    }
    /// Unfocus the current widget
    fn unfocus(&mut self) {
        self.internal.set_focused(false);
    }

    fn step_focus(&mut self, _back: bool) -> bool {
        self.internal.toggle_focus();
        self.internal.focused()
    }
}

impl WidgetGrab for Button {
    /// Grab for a window state
    fn grab(&mut self) -> bool {
        if !self.internal.grabbed() {
            self.internal.set_grab(true);
            return true;
        }
        false
    }
    /// Ungrab from a window state
    fn ungrab(&mut self) -> bool {
        if self.internal.grabbed() {
            self.internal.set_grab(false)
        }
        false
    }
}