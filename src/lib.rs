#![allow(dead_code)]

pub mod binding;
pub mod container;
pub mod decorator;
pub mod layout;
pub mod state;
pub mod widget;

pub trait Boxed {
    #[inline]
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}
