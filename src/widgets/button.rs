use crate::widget::{Widget, WidgetInternal, Boundaries};
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
    fn handle(&mut self, mouse: &MouseState, key: &KeyState) {
        if mouse.clicked() && point_on_area!(mouse.coordinates_relative(), self.get_bounds()) {
            self.clicked = true;
        }
        if self.clicked && !mouse.clicked() {
            println!("CLICKED BUTTON: {} {:?} {:?} ({:?})", self.label, mouse.coordinates_relative(), 
            self.get_bounds(), key.pressed());
            self.clicked = false;
        }
    }

    fn get_bounds(&self) -> Boundaries {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: Boundaries) {
        self.internal.set_boundaries(bounds);
    }
}