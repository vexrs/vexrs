#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

extern crate vexv5rt;

// Use this so any crates using ceros-core will
// automatically use the newlib-allocator
// that works with libv5rt.
#[cfg(feature="alloc")]
mod alloc_a;

// We also want users to use our panic handler
#[cfg(feature="panic")]
mod panic;

// This contains a few utilities for use in user code.