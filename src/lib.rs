use std::mem;

struct DefaultDelete<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> DefaultDelete<T> {
    fn new() -> Self {
        DefaultDelete { _marker: std::marker::PhantomData }
    }
}

impl<T> Default for DefaultDelete<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct UniquePtr<T, Deleter = DefaultDelete<T>> {
    ptr: *mut T,
    deleter: Deleter,
}

impl<T> UniquePtr<T> {
    fn new() -> Self {
        UniquePtr { ptr: std::ptr::null_mut(), deleter: DefaultDelete::new() }
    }

    fn with_ptr(ptr: *mut T) -> Self {
        UniquePtr { ptr, deleter: DefaultDelete::new() }
    }
}

impl<T, Deleter> Drop for UniquePtr<T, Deleter> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}

impl<T, Deleter> Clone for UniquePtr<T, Deleter>
where
    Deleter: Clone,
{
    fn clone(&self) -> Self {
        Self::with_ptr_and_deleter(self.ptr, self.deleter.clone())
    }
}

impl<T, Deleter> UniquePtr<T, Deleter> {
    fn reset(&mut self, ptr: *mut T) {
        unsafe {
            Box::from_raw(self.ptr);
        }
        self.ptr = ptr;
    }

    fn release(&mut self) -> *mut T {
        let old_ptr = self.ptr;
        self.ptr = std::ptr::null_mut();
        old_ptr
    }

    fn swap(&mut self, other: &mut Self) {
        mem::swap(&mut self.ptr, &mut other.ptr);
        mem::swap(&mut self.deleter, &mut other.deleter);
    }

    fn get(&self) -> *mut T {
        self.ptr
    }

    fn with_ptr_and_deleter(ptr: *mut T, deleter: Deleter) -> Self {
        UniquePtr { ptr, deleter }
    }
}

impl<T, Deleter> std::ops::Deref for UniquePtr<T, Deleter> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T, Deleter> std::ops::DerefMut for UniquePtr<T, Deleter> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T, Deleter> UniquePtr<T, Deleter> {
    fn get_deleter(&self) -> &Deleter {
        &self.deleter
    }
}

impl<T, Deleter> From<UniquePtr<T, Deleter>> for Option<Box<T>> {
    fn from(ptr: UniquePtr<T, Deleter>) -> Self {
        if ptr.ptr.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(ptr.ptr) })
        }
    }
}

impl<T> UniquePtr<T, DefaultDelete<T>> {
    fn into_raw(self) -> *mut T {
        let ptr = self.ptr;
        mem::forget(self);
        ptr
    }

    fn from_raw(ptr: *mut T) -> Self {
        UniquePtr::with_ptr(ptr)
    }
}