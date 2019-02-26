use std::marker::PhantomData;
// TODO: recreate binding

pub trait Binding<T> {
    fn proxy(&self) -> BindProxy<T>;
}

pub struct PointerBinding<'a, T: 'a> {
    ptr: *const T,
    phantom: PhantomData<&'a T>
}

pub struct BoxBinding<T> {
    data: Box<T>
}

pub struct BindProxy<T> {
    ptr: *const T
}

impl <T> BindProxy<T> {
    pub fn act<F>(&self, func: F) where F: Fn(&T) {
        unsafe {
            func(&*self.ptr);
        }
    }

    pub fn act_mut<F>(&self, func: F) where F: Fn(&mut T) {
        unsafe {
            func(&mut *( self.ptr as *mut T ));
        }
    }
}

impl <'a, T> PointerBinding<'a, T> {
    pub fn new(ptr: &'a T) -> Self {
        PointerBinding {
            ptr: ptr as *const T,
            phantom: PhantomData
        }
    }
}

impl <'a, T> Binding<T> for PointerBinding<'a, T> {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: self.ptr as *const T
        }
    }
}

impl <T> BoxBinding<T> {
    fn new(val: T) -> Self {
        BoxBinding {
            data: Box::new(val)
        }
    }
}

impl <T> Binding<T> for BoxBinding<T> {
    fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: self.data.as_ref() as *const T
        }
    }
}