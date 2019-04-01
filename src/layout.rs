use crate::widget::{Dimensions, WidgetInternal};
use crate::Boxed;

pub trait Layout<P, D> {
    /// Applies the layout to the widget list of a Container
    fn layout(&self, internal_list: &mut [WidgetInternal<P, D>], dim: &Dimensions<D>);
    /// Calculates the minimum dimensions using the widget list of a Container
    fn minimum_size(&self, internal_list: &[WidgetInternal<P, D>]) -> Dimensions<D>;
}

/// Layout with empty implementation, useful for widgets with fixed position and dimensions
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
