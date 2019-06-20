//! Single-Threaded pointers that avoids the borrow checker for use inside widgets
use crate::group::{push_event, GroupEvent::Signal, GroupID};

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

pub type Opaque = usize;

/// Create a new BindProxy using a Trait implementation
pub trait Binding<T> {
    /// Prepare and Create a new BindProxy
    fn proxy(&self) -> BindProxy<T>;
    /// Converts a reference into a raw pointer with no type
    fn opaque(&self) -> Opaque;
}

/// Cast an opaque pointer into a mutable reference of T type, use with caution!
#[inline]
pub unsafe fn cast_opaque<'a, T>(opaque: Opaque) -> &'a mut T {
    &mut *(opaque as *mut T)
}

/// Implementation of BindProxy for References
impl<T> Binding<T> for &'_ T {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: *self as *const T,
        }
    }

    #[inline]
    fn opaque(&self) -> Opaque {
        *self as *const T as Opaque
    }
}

/// Implementation of BindProxy for Box<T>
impl<T> Binding<T> for Box<T> {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: self.as_ref() as *const T,
        }
    }

    #[inline]
    fn opaque(&self) -> Opaque {
        self.as_ref() as *const T as Opaque
    }
}
