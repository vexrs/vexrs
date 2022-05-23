#![no_std]

extern crate alloc;

/// A panic handler implementation
mod panic;

/// Registers the newlib allocator as the default rust allocator
mod allocator;

/// The automatically generated libv5rt bindings
pub mod libv5rt;

/// The core CEROS runtime.
pub mod runtime;