//! Trait for control a renderer on a Container

use crate::widget::WidgetInternal;
use crate::Boxed;

/// Decorator Trait for a Container
pub trait Decorator<P, D> {
    /// Prepares the decorator before draw the widgets
    fn before(&mut self, internal: &WidgetInternal<P, D>);
    /// Finalizes the decorator after draw the widgets
    fn after(&mut self, internal: &WidgetInternal<P, D>);
    /// Update the status of the decorator, this is called when the layout of a container is changed
    fn update(&mut self, internal: &WidgetInternal<P, D>);
}

/// Decorator with empty implementation, useful for root container
pub struct EmptyDecorator;

impl<P, D> Decorator<P, D> for EmptyDecorator {
    fn before(&mut self, _: &WidgetInternal<P, D>) {}
    fn after(&mut self, _: &WidgetInternal<P, D>) {}
    fn update(&mut self, _: &WidgetInternal<P, D>) {}
}

impl Boxed for EmptyDecorator {}
