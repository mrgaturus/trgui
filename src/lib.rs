#![allow(dead_code)]

pub mod binding;
pub mod state;
pub mod widget;
pub mod layout;
pub mod decorator;
pub mod container;

pub trait Boxed {
    #[inline]
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}