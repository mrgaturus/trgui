use crate::widget::{WidgetInternal};
use crate::Boxed;

pub trait Layout {
    fn layout(&self, _: &mut [WidgetInternal], _: &(i32, i32)) {}
    fn minimum_size(&self, _: &[WidgetInternal]) -> (i32, i32) { (0, 0) }
}

pub struct EmptyLayout;

impl Layout for EmptyLayout {}
impl Boxed for EmptyLayout {}