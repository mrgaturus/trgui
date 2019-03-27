use std::marker::PhantomData;

pub struct BindProxy<T> {
    ptr: *const T,
}

impl<T> BindProxy<T> {
    pub fn read(&self) -> &T {
        unsafe { &*self.ptr }
    }

    pub fn write<F>(&self, func: F)
    where
        F: Fn(&mut T),
    {
        unsafe {
            func(&mut *(self.ptr as *mut T));
        }
    }

    pub unsafe fn write_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }
}

pub trait Binding<T> {
    fn proxy(&self) -> BindProxy<T>;
}

pub struct PointerBinding<'a, T: 'a> {
    ptr: *const T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> PointerBinding<'a, T> {
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
