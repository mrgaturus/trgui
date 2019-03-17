use crate::widget::{WidgetInternal};

pub trait Decorator {
    fn before(&mut self, _: &WidgetInternal) {}
    fn after(&mut self, _: &WidgetInternal) {}
    fn update(&mut self, _: &WidgetInternal) {}
}

pub struct EmptyDecorator;

impl Decorator for EmptyDecorator {

}