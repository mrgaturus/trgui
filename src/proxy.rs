// FIXME: figure how to use something like Pin

//! Single-Threaded pointers that avoids the borrow checker for use inside widgets
use crate::group::{push_event, GroupEvent::Signal, GroupID};

/// Shares external data to (not limited to) widgets.
///
/// A RefProxy is a Single-Threaded Pointer without borrow checker for the widgets.
/// it only can store types without lifetime parameters, if you type has lifetime
/// parameters, use unsafe opaque pointer instead. This may cause UB if you dropped
/// the pointed data first than the proxy or point a constant.
pub struct RefProxy<T> {
    ptr: *mut T,
}

impl<T> RefProxy<T> {
    /// Returns a non-mutable reference
    #[inline]
    pub fn read(&self) -> &T {
        unsafe { &*self.ptr }
    }

    #[inline]
    /// Returns a mutable reference
    pub unsafe fn read_write(&self) -> &mut T {
        &mut *self.ptr
    }

    #[inline]
    /// Returns a mutable reference and push a signal id into queue
    pub unsafe fn rw_push(&self, id: GroupID) -> &mut T {
        push_event(Signal(id));

        &mut *self.ptr
    }
}

pub type Opaque = *mut u8;

/// Create a new RefProxy using a Trait implementation
pub trait Proxy<T> {
    /// Prepare and Create a new RefProxy
    fn proxy(&mut self) -> RefProxy<T>;
    /// Converts a reference into a raw pointer with no type
    fn opaque(&mut self) -> Opaque;
}

/// Cast an opaque pointer into a mutable reference of T type, use with caution!
#[inline]
pub unsafe fn cast_opaque<'a, T>(opaque: Opaque) -> &'a mut T {
    &mut *(opaque as *mut T)
}

/// Implementation of Proxy for References
impl<T> Proxy<T> for &'_ mut T {
    fn proxy(&mut self) -> RefProxy<T> {
        RefProxy {
            ptr: *self as *mut T,
        }
    }

    #[inline]
    fn opaque(&mut self) -> Opaque {
        *self as *const T as Opaque
    }
}

/// Implementation of Proxy for Box<T>
impl<T> Proxy<T> for Box<T> {
    fn proxy(&mut self) -> RefProxy<T> {
        RefProxy {
            ptr: self.as_mut() as *mut T,
        }
    }

    #[inline]
    fn opaque(&mut self) -> Opaque {
        self.as_ref() as *const T as Opaque
    }
}
