use crate::widget::{WidgetInternal};
use crate::Boxed;

pub trait Decorator {
    fn before(&mut self, _: &WidgetInternal) {}
    fn after(&mut self, _: &WidgetInternal) {}
    fn update(&mut self, _: &WidgetInternal) {}
}

pub struct EmptyDecorator;

impl Decorator for EmptyDecorator {}
impl Boxed for EmptyDecorator {}