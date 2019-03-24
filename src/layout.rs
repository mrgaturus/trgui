use crate::widget::{WidgetInternal, Dimensions};
use crate::Boxed;

pub trait Layout<P, D> {
    fn layout(&self, _: &mut [WidgetInternal<P, D>], _: &Dimensions<D>) {}
    fn minimum_size(&self, _: &[WidgetInternal<P, D>]) -> Dimensions<D>;
}

pub struct EmptyLayout;

impl <P, D> Layout<P, D> for EmptyLayout where D: From<u8> {
    fn minimum_size(&self, _: &[WidgetInternal<P, D>]) -> Dimensions<D> {
        (D::from(0 as u8), D::from(0 as u8))
    }
}
impl Boxed for EmptyLayout {}