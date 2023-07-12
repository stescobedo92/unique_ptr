struct DefaultDeleter;

impl<T> DefaultDeleter {
    fn delete(ptr: *mut T) {
        unsafe {
            std::ptr::drop_in_place(ptr);
        }
    }
}