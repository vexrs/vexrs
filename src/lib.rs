#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

extern crate alloc;

/// A panic handler implementation
mod panic;

/// Registers the newlib allocator as the default rust allocator
mod allocator;

/// The automatically generated libv5rt bindings
pub mod libv5rt;

/// The core CEROS runtime.
pub mod runtime;
pub use runtime::RUNTIME;

/// Synchronization primitives that build on top of the runtime
pub mod sync;

/// A serial writer implementation
pub mod serial;