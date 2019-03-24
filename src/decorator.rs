use crate::widget::{WidgetInternal};
use crate::Boxed;

pub trait Decorator<P, D> {
    fn before(&mut self, _: &WidgetInternal<P, D>) {}
    fn after(&mut self, _: &WidgetInternal<P, D>) {}
    fn update(&mut self, _: &WidgetInternal<P, D>) {}
}

pub struct EmptyDecorator;

impl <P, D> Decorator<P, D> for EmptyDecorator {}
impl Boxed for EmptyDecorator {}