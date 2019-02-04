use std::marker::PhantomData;

pub struct PointerBinding<'a, T: 'a> {
    ptr: (bool, *const T),
    phantom: PhantomData<&'a T>
}

pub struct BindProxy<T> {
    ptr: *mut (bool, *const T)
}

impl <T> BindProxy<T> {
    pub fn act<F>(&self, func: F) where F: Fn(&T) {
        unsafe {
            if (*self.ptr).0 {
                (*self.ptr).0 = false;
                func(&*( (*self.ptr).1 ));
                (*self.ptr).0 = true;
            }
        }
    }

    pub fn act_mut<F>(&self, func: F) where F: Fn(&mut T) {
        unsafe {
            if (*self.ptr).0 {
                (*self.ptr).0 = false;
                func(&mut *( (*self.ptr).1 as *mut T ));
                (*self.ptr).0 = true;
            }
        }
    }

    #[inline]
    pub fn usable(&self) -> bool {
        unsafe {
            (*self.ptr).0
        }
    }
}

impl <'a, T: 'a> PointerBinding<'a, T> {
    pub fn new(ptr: &'a T) -> Self {
        PointerBinding {
            ptr: (true, ptr),
            phantom: PhantomData
        }
    }

    pub fn proxy(&self) -> BindProxy<T> {
        BindProxy {
            ptr: &self.ptr as *const _ as *mut (bool, *const T)
        }    
    }

    #[inline]
    pub fn usable(&self) -> bool {
        self.ptr.0
    }

    pub fn set_usable(&mut self, usable: bool) {
        self.ptr.0 = usable;
    }
}