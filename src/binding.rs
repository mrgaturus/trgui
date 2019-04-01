//! Single-Threaded pointers that avoids the borrow checker for use on widgets

use std::marker::PhantomData;

/// Shares external data to (not limited to) widgets. 
/// 
/// A BindProxy is a Single-Threaded Pointer without borrow checker for the widgets.
pub struct BindProxy<T> {
    ptr: *const T,
}


impl<T> BindProxy<T> {
    /// Returns a non-mutable safe reference
    pub fn read(&self) -> &T {
        unsafe { &*self.ptr }
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

    /// Returns a raw pointer of the data
    pub unsafe fn write_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }
}

/// Create a new BindProxy using a Trait implementation
pub trait Binding<T> {
    /// Prepare and Create a new BindProxy
    fn proxy(&self) -> BindProxy<T>;
}

/// Quasi-safe binding to any pointer
pub struct PointerBinding<'a, T: 'a> {
    ptr: *const T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> PointerBinding<'a, T> {
    /// Creates a new binding from any pointer. the pointed data should 
    /// live long than the binding
    pub fn new(ptr: &'a mut T) -> Self {
        PointerBinding {
            ptr: ptr as *const T,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Binding<T> for PointerBinding<'a, T> {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: self.ptr as *const T,
        }
    }
}

impl<T> Binding<T> for Box<T> {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: self.as_ref() as *const T,
        }
    }
}
