use crate::container::BoundList;
use crate::widget::{Dimensions};

pub trait Layout {
    fn layout(&self, widgets: &mut BoundList, dimensions: &Dimensions);
    fn minimum_size(&self, widgets: &BoundList) -> Dimensions;

    #[inline]
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}

pub struct FixedLayout { 
    clamp: bool
}

impl FixedLayout {
    pub fn new(clamp: bool) -> Self {
        FixedLayout {
            clamp
        }
    }
}

impl Layout for FixedLayout {
    fn layout(&self, widgets: &mut BoundList, dimensions: &Dimensions) {
        for widget in widgets.iter_mut() {
            let widget_bounds = widget.boundaries();
            if self.clamp {
                let mut new_pos = (widget_bounds.0, widget_bounds.1);
                
                new_pos.0 = if widget_bounds.0 < 0 {
                    0
                } else if widget_bounds.0 + widget_bounds.2 > dimensions.0 {
                    dimensions.0 - widget_bounds.2
                } else {
                    widget_bounds.0
                };

                new_pos.1 = if widget_bounds.1 < 0 {
                    0
                } else if widget_bounds.1 + widget_bounds.3 > dimensions.1 {
                    dimensions.1 - widget_bounds.3
                } else {
                    widget_bounds.1
                };

                widget.set_coords(new_pos.0, new_pos.1);
            } else {
                if widget_bounds.0 < 0 || widget_bounds.0 + widget_bounds.2 > dimensions.0 ||
                    widget_bounds.1 < 0 || widget_bounds.1 + widget_bounds.3 > dimensions.1 {
                        widget.set_visible_layout(false);
                    } else {
                        widget.set_visible_layout(true);
                    }
            }
        }
    }

    fn minimum_size(&self, _widgets: &BoundList) -> Dimensions {
        (0, 0)
    }
}