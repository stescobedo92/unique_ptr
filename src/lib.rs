struct DefaultDeleter;

impl<T> DefaultDeleter {
    fn delete(ptr: *mut T) {
        unsafe {
            std::ptr::drop_in_place(ptr);
        }
    }
}

struct UniquePtr<T> {
    ptr: *mut T,
    deleter: Box<dyn FnMut(*mut T)>,
}

impl<T> UniquePtr<T> {
    fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            deleter: Box::new(DefaultDeleter::delete),
        }
    }

    fn new_with_deleter(ptr: *mut T, deleter: impl FnMut(*mut T)) -> Self {
        Self {
            ptr,
            deleter: Box::new(deleter),
        }
    }

    fn drop(self) {
        (self.deleter)(self.ptr);
    }

    fn reset(&mut self, ptr: *mut T) {
        (self.deleter)(self.ptr);
        self.ptr = ptr;
    }

    fn release(&mut self) -> *mut T {
        let ptr = self.ptr;
        self.ptr = std::ptr::null_mut();
        ptr
    }

    fn swap(&mut self, other: &mut Self) {
        std::mem::swap(self.ptr, other.ptr);
        std::mem::swap(self.deleter, other.deleter);
    }

    fn get(&self) -> *mut T {
        self.ptr
    }

    fn get_deleter(&self) -> &dyn FnMut(*mut T) {
        &*self.deleter
    }
}