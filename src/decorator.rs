use crate::widget::{WidgetInternal};
use crate::Boxed;

pub trait Decorator {
    fn before(&mut self, _: &WidgetInternal) {}
    fn after(&mut self, _: &WidgetInternal) {}
    fn update(&mut self, _: &WidgetInternal) {}

    #[inline]
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}

pub struct EmptyDecorator;

impl Decorator for EmptyDecorator {}
impl Boxed for EmptyDecorator {}