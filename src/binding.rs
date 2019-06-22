// FIXME: figure how to use something like Pin

//! Single-Threaded pointers that avoids the borrow checker for use inside widgets
use crate::group::{push_event, GroupEvent::Signal, GroupID};

/// Shares external data to (not limited to) widgets.
///
/// A BindProxy is a Single-Threaded Pointer without borrow checker for the widgets.
/// it only can store types without lifetime parameters, if you type has lifetime
/// parameters, use unsafe opaque pointer instead.
pub struct BindProxy<T> {
    ptr: *const T,
}

impl<T> BindProxy<T> {
    /// Returns a non-mutable reference
    #[inline]
    pub unsafe fn read(&self) -> &T {
        &*self.ptr
    }

    #[inline]
    /// Returns a mutable reference
    pub unsafe fn read_write(&self) -> &mut T {
        &mut *(self.ptr as *mut T)
    }

    #[inline]
    /// Returns a mutable reference and push a signal id into queue
    pub unsafe fn rw_push(&self, id: GroupID) -> &mut T {
        push_event(Signal(id));

        &mut *(self.ptr as *mut T)
    }
}

pub type Opaque = *mut u8;

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
