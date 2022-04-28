#![no_std]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(asm_const)]
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
pub mod util;

// This module contains the main CEROS green threads runtime
#[cfg(feature="green_threads")]
pub mod runtime;

// Make the main function from the runtime public
pub use runtime::main;

// This module contains a utility handler for printing to the serial port
// using standard rust macros.
pub mod print;

// This crates prelude contains all the types and functions that are
// commonly used in user code.
pub mod prelude;