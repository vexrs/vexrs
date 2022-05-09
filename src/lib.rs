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
pub mod util;

// This module contains the main CEROS green threads runtime
#[cfg(feature="green_threads")]
pub mod runtime;

// Make the main function from the runtime public
pub use runtime::main;

// Abstractions over libv5rt
pub mod hardware;


// This module contains a utility handler for printing to the serial port
// using standard rust macros.
#[cfg(feature="print")]
pub mod print;

// This crates prelude contains all the types and functions that are
// commonly used in user code.
pub mod prelude;

// The system library containing all system utilities
mod system;

// Export the ceros_serial main macro
pub use ceros_macro::ceros_main;

// Export the macros in alloc
pub use alloc::{format, vec};



// Global singletons
lazy_static::lazy_static! {
    static ref RUNTIME: runtime::Runtime = runtime::Runtime::new();
    static ref DISPLAY: runtime::mutex::Mutex<hardware::display::Display> = runtime::mutex::Mutex::new(hardware::display::Display::new());
    static ref DEVICE_MANAGER: runtime::mutex::Mutex<hardware::devices::manager::DeviceManager> = runtime::mutex::Mutex::new(hardware::devices::manager::DeviceManager::new());
}