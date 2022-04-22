#![no_std]
#![feature(default_alloc_error_handler)]

// Use this so any crates using ceros-core will
// automatically use the newlib-allocator
// that works with libv5rt.
#[cfg(feature="alloc")]
mod alloc_a;

// We also want users to use our panic handler
#[cfg(feature="panic")]
mod panic;