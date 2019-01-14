// TODO: Implement unique text field functions for this widget. As this stands for now,
// it's mostly an adaption of the button widget.
use crate::widget::{Widget, WidgetInternal, WidgetBounds}

pub struct Field {
    label: String,
    internal: WidgetInternal
}

impl Field {
    pub fn new(label: &str, bounds: (usize, usize, usize, usize)) -> Self {
        Field {
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

impl Widget for Field {
    /// Draw the current widget
    fn draw(&self) {
        println!("FIELD: ({}) ({:?})", self.label, self.get_bounds());
    }
    /// Update the status of widget
    fn update(&mut self) {

    }
    /// Handle an event state
    fn handle(&mut self, mouse: &MouseState, key: &KeyState) {
        if mouse.clicked() && point_on_area!(mouse.coordinates(), self.get_bounds()) {
            println!("CLICKED FIELD: {} ({:?}) ({:?}) ({:?})", self.label, mouse.coordinates(), 
            self.get_bounds(), key.pressed());
        }
    }
}

impl WidgetBounds for Field {
    type Dim = usize;

    fn get_bounds(&self) -> (Self::Dim, Self::Dim, Self::Dim, Self::Dim) {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: (Self::Dim, Self::Dim, Self::Dim, Self::Dim)) {
        self.internal.set_boundaries(bounds);
    }
}