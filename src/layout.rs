use crate::widget::{WidgetInternal, Dimensions};
use crate::Boxed;

pub trait Layout {
    fn layout(&self, _: &mut [WidgetInternal], _: &Dimensions) {}
    fn minimum_size(&self, _: &[WidgetInternal]) -> Dimensions { (0, 0) }
}

pub struct EmptyLayout;

impl Layout for EmptyLayout {}
impl Boxed for EmptyLayout {}