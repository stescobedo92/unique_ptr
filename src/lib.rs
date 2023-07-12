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