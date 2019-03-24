use crate::widget::{Dimensions, WidgetInternal};
use crate::Boxed;

pub trait Layout<P, D> {
    fn layout(&self, internal_list: &mut [WidgetInternal<P, D>], dim: &Dimensions<D>);
    fn minimum_size(&self, internal_list: &[WidgetInternal<P, D>]) -> Dimensions<D>;
}

pub struct EmptyLayout;

impl<P, D> Layout<P, D> for EmptyLayout
where
    D: Default,
{
    fn layout(&self, _: &mut [WidgetInternal<P, D>], _: &Dimensions<D>) {}

    fn minimum_size(&self, _: &[WidgetInternal<P, D>]) -> Dimensions<D> {
        (Default::default(), Default::default())
    }
}
impl Boxed for EmptyLayout {}
