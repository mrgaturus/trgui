use crate::container::WidgetList;
use crate::widget::Boundaries;

pub trait Layout {
    fn layout(&mut self, widgets: &mut WidgetList, bounds: &Boundaries);

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
    fn layout(&mut self, widgets: &mut WidgetList, bounds: &Boundaries) {

        for widget in widgets.iter_mut() {
            if self.clamp {
                let widget_bounds = widget.get_bounds();
                let mut new_pos = (widget_bounds.0, widget_bounds.1);
                
                new_pos.0 = if widget_bounds.0 < 0 {
                    0
                } else if widget_bounds.0 + widget_bounds.2 > bounds.2 {
                    bounds.2 - widget_bounds.2
                } else {
                    widget_bounds.0
                };

                new_pos.1 = if widget_bounds.1 < 0 {
                    0
                } else if widget_bounds.1 + widget_bounds.3 > bounds.3 {
                    bounds.3 - widget_bounds.3
                } else {
                    widget_bounds.1
                };

                widget.set_pos(new_pos);
            }
        }
    }
}