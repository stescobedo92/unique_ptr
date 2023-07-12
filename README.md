# UniquePtr

The `UniquePtr` library in Rust is inspired by the `unique_ptr` smart pointer in C++. Both implementations share the same fundamental purpose: providing unique ownership of dynamically allocated objects and ensuring proper deallocation.

Here are some key similarities and differences between the two:

1. Unique Ownership: Both `UniquePtr` in Rust and `unique_ptr` in C++ enforce exclusive ownership of the managed object. This means that only one smart pointer instance can own and manage the object at any given time.
2. Automatic Deallocation: When the `UniquePtr` or `unique_ptr` instance goes out of scope or is explicitly reset, they automatically deallocate the managed object, freeing the associated memory.

## Usage

```rust
use unique_ptr::UniquePtr;

fn main() {
    let ptr = Box::new(42);
    let unique_ptr = UniquePtr::with_ptr(ptr);

    assert_eq!(unsafe { *unique_ptr.ptr }, 42);

    let ptr2 = Box::new(99);
    unique_ptr.reset(ptr2);

    assert_eq!(unsafe { *unique_ptr.ptr }, 99);
}
```


## Features

* Supports `Clone`.
* Supports `Deref` and `DerefMut`.
* Can be used with any type that implements the `Drop` trait.
* Supports custom deleters using the `Deleter` type parameter.

- Implements common smart pointer operations such as resetting the pointer, releasing ownership, cloning the pointer, and dereferencing.
- Provides a safe interfacefor working with raw pointers.

## License

This project is licensed under the MIT license.
