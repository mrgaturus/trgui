//! Single-Threaded pointers that avoids the borrow checker for use on widgets
use crate::group::{GroupEvent::Signal, GroupID, push_event};

/// Shares external data to (not limited to) widgets. 
/// 
/// A BindProxy is a Single-Threaded Pointer without borrow checker for the widgets.
pub struct BindProxy<T> {
    ptr: *const T,
}

impl<T> BindProxy<T> {
    /// Returns a non-mutable safe reference
    #[inline]
    pub fn read(&self) -> &T {
        unsafe { &*self.ptr }
    }

    #[inline]
    /// Returns a raw pointer of the data
    pub unsafe fn write_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }

    /// Write on the data using a non-capturing closure with a mutable 
    /// reference as an argument
    pub fn write<F>(&self, func: F)
    where
        F: Fn(&mut T),
    {
        unsafe {
            func(&mut *(self.ptr as *mut T));
        }
    }

    /// Write on the data using a non-capturing closure with a mutable 
    /// reference as an argument and push a signal to the event queue
    pub fn write_push<F>(&self, func: F, id: GroupID)
    where
        F: Fn(&mut T),
    {
        unsafe {
            func(&mut *(self.ptr as *mut T));
        }

        push_event(Signal(id));
    }
}

/// Create a new BindProxy using a Trait implementation
pub trait Binding<T> {
    /// Prepare and Create a new BindProxy
    fn proxy(&self) -> BindProxy<T>;
}

/// Implementation of BindProxy for References
impl<T> Binding<T> for &'_ T {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: *self as *const T,
        }
    }
}

/// Implementation of BindProxy for Box<T>
impl<T> Binding<T> for Box<T> {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: self.as_ref() as *const T,
        }
    }
}
