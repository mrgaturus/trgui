use crate::widget::{Widget, WidgetInternal, Boundaries, FocusAction, WidgetGrab};
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
    fn handle_mouse(&mut self, mouse: &MouseState, grab_ptr: &mut Option<*mut Widget>) -> FocusAction {
        if mouse.clicked() {
            println!("{} {} {:?} {:?}", "Clicked", self.label, mouse.coordinates_relative(), mouse.coordinates());
            self.clicked = true;
            unsafe {
                self.grab(grab_ptr);
            }
            return FocusAction::Ok;
        } else if self.clicked {
            self.clicked = false;
            unsafe {
                self.ungrab(grab_ptr);
            }
        } else {
            println!("{} {} {:?} {:?}", "Hovered", self.label, mouse.coordinates_relative(), mouse.coordinates());
        }

        FocusAction::False
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
    fn focus(&mut self, _back: bool) -> FocusAction {
        println!("Focused BUTTON: {}", self.label);
        FocusAction::Ok
    }
    /// Unfocus the current widget
    fn unfocus(&mut self) {
        println!("Un Focused BUTTON: {}", self.label);
    }
}

impl WidgetGrab for Button {
    /// Grab for a window state
    unsafe fn grab(&mut self, grab_ptr: &mut Option<*mut Widget>) {
        if !self.internal.grabbed() {
            if grab_ptr.is_none() {
                self.internal.set_grab(true);
                *grab_ptr = Some(self as *mut Widget)
            }
        }
    }
    /// Ungrab from a window state
    unsafe fn ungrab(&mut self, grab_ptr: &mut Option<*mut Widget>) {
        if self.internal.grabbed() {
            self.internal.set_grab(false);
            *grab_ptr = None;
        }
    }
}