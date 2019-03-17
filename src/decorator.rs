use crate::widget::{WidgetInternal};

pub const DECORATOR_BEFORE: u16 =  0b10000000000;
pub const DECORATOR_AFTER:  u16 =  0b100000000000;
pub const DECORATOR_UPDATE: u16 =  0b1000000000000;
pub const DECORATOR_ALL: u16 = DECORATOR_BEFORE | DECORATOR_AFTER | DECORATOR_UPDATE;

pub trait Decorator {
    fn before(&mut self, _: &WidgetInternal) {}
    fn after(&mut self, _: &WidgetInternal) {}
    fn update(&mut self, _: &WidgetInternal) {}
}

pub struct EmptyDecorator;

impl Decorator for EmptyDecorator {

}