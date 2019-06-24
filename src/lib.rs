#![allow(dead_code)]

pub mod group;
pub mod proxy;
pub mod state;
pub mod widget;

mod container;
pub use crate::container::Container;

use crate::widget::{Dimensions, WidgetInternal};

/// Decorator Trait for a Container
pub trait Decorator<T, CTX: Sized> {
    /// Prepares the decorator before draw the widgets
    fn before(&mut self, internal: &WidgetInternal<T>, ctx: &mut CTX);
    /// Finalizes the decorator after draw the widgets
    fn after(&mut self, internal: &WidgetInternal<T>, ctx: &mut CTX);
    /// Update the status of the decorator, this is called when the layout of a container is changed
    fn update(&mut self, internal: &WidgetInternal<T>);
}

/// Layout Trait for a Container
pub trait Layout<T> {
    /// Applies the layout to the widget list of a Container
    fn layout(&self, internal_list: &mut [WidgetInternal<T>], c_internal: &WidgetInternal<T>);
    /// Calculates the minimum dimensions using the widget list of a Container
    fn min_dimensions(&self, internal_list: &[WidgetInternal<T>]) -> Dimensions<T>;
}

pub mod empty {
    //! Empty Implementations
    use crate::widget::{Dimensions, WidgetInternal};
    use crate::{Decorator, Layout};

    /// Layout with empty implementation
    pub struct EmptyLayout;

    impl<T: Default> Layout<T> for EmptyLayout {
        fn layout(&self, _: &mut [WidgetInternal<T>], _: &WidgetInternal<T>) {}

        fn min_dimensions(&self, _: &[WidgetInternal<T>]) -> Dimensions<T> {
            (Default::default(), Default::default())
        }
    }

    /// Decorator with empty implementation
    pub struct EmptyDecorator;

    impl<T, CTX: Sized> Decorator<T, CTX> for EmptyDecorator {
        fn before(&mut self, _: &WidgetInternal<T>, _: &mut CTX) {}
        fn after(&mut self, _: &WidgetInternal<T>, _: &mut CTX) {}
        fn update(&mut self, _: &WidgetInternal<T>) {}
    }
}
