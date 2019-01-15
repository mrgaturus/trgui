use crate::widget::{Widget, WidgetInternal, WidgetBounds};
use crate::state::{KeyState, MouseState};

pub struct Button {
    label: String,
    internal: WidgetInternal
}

impl Button {
    pub fn new(label: &str, bounds: (usize, usize, usize, usize)) -> Self {
        Button {
            label: String::from(label),
            internal: WidgetInternal::new(bounds)
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
        if mouse.clicked() && point_on_area!(mouse.coordinates(), self.get_bounds()) {
            println!("CLICKED BUTTON: {} ({:?}) ({:?}) ({:?})", self.label, mouse.coordinates(), 
            self.get_bounds(), key.pressed());
        }
    }
}

impl WidgetBounds for Button {
    type Dim = usize;

    fn get_bounds(&self) -> (Self::Dim, Self::Dim, Self::Dim, Self::Dim) {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: (Self::Dim, Self::Dim, Self::Dim, Self::Dim)) {
        self.internal.set_boundaries(bounds);
    }
}