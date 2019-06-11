#![allow(dead_code)]

pub mod binding;
pub mod group;
pub mod state;
pub mod widget;

mod container;
pub use crate::container::Container;

use crate::widget::{Dimensions, Widget, WidgetInternal};
use std::ops::Add;

/// Decorator Trait for a Container
pub trait Decorator<P, D> {
    /// Prepares the decorator before draw the widgets
    fn before(&mut self, internal: &WidgetInternal<P, D>);
    /// Finalizes the decorator after draw the widgets
    fn after(&mut self, internal: &WidgetInternal<P, D>);
    /// Update the status of the decorator, this is called when the layout of a container is changed
    fn update(&mut self, internal: &WidgetInternal<P, D>);
}

/// Layout Trait for a Container
pub trait Layout<P, D> {
    /// Applies the layout to the widget list of a Container
    fn layout(&self, internal_list: &mut [WidgetInternal<P, D>], dim: &Dimensions<D>);
    /// Calculates the minimum dimensions using the widget list of a Container
    fn minimum_size(&self, internal_list: &[WidgetInternal<P, D>]) -> Dimensions<D>;
}

pub mod empty {
    //! Empty Implementations
    use crate::widget::{Dimensions, WidgetInternal};
    use crate::{Decorator, Layout};

    /// Layout with empty implementation
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

    /// Decorator with empty implementation
    pub struct EmptyDecorator;

    impl<P, D> Decorator<P, D> for EmptyDecorator {
        fn before(&mut self, _: &WidgetInternal<P, D>) {}
        fn after(&mut self, _: &WidgetInternal<P, D>) {}
        fn update(&mut self, _: &WidgetInternal<P, D>) {}
    }
}

/// Prepare the widget to be boxed
pub trait Boxed<T: ?Sized> {
    #[inline]
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<P, D, W: Widget<P, D>> Boxed<Widget<P, D>> for W
where
    D: Sized + Copy + Clone + PartialOrd + Default,
    P: Sized + Copy + Clone + Add<Output = P> + PartialOrd + From<D> + Default,
{
}

impl<P, D, L: Layout<P, D>> Boxed<Layout<P, D>> for L {}
