#![allow(dead_code)]

pub mod binding;
mod container;
pub mod decorator;
pub mod layout;
pub mod signal;
pub mod state;
pub mod widget;

pub use crate::container::Container;

/// Prepare the widget to be boxed
pub trait Boxed {
    #[inline]
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
