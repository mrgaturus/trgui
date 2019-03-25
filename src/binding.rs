use std::collections::VecDeque;
use std::marker::PhantomData;

// UNSAFE STATIC QUEUE

pub type BindID = u32;

static mut BIND_QUEUE: Option<VecDeque<BindID>> = None;

pub fn queue_id(id: BindID) {
    unsafe {
        if let Some(ref mut queue) = BIND_QUEUE {
            if !queue.contains(&id) {
                queue.push_back(id);
            }
        } else {
            let mut queue: VecDeque<BindID> = VecDeque::with_capacity(4);
            queue.push_back(id);

            BIND_QUEUE = Some(queue);
        }
    }
}

pub fn next_id() -> Option<BindID> {
    unsafe {
        if let Some(ref mut queue) = BIND_QUEUE {
            queue.pop_front()
        } else {
            None
        }
    }
}

// END UNSAFE STATIC QUEUE

#[derive(Copy, Clone)]
pub enum BindType {
    Any,
    ID(BindID),
    None,
}

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
