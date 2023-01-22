#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

extern crate alloc;
extern crate core;


/// A panic handler implementation
mod panic;

/// Registers the newlib allocator as the default rust allocator
mod allocator;

mod runtime;

/// The automatically generated libv5rt bindings
pub mod libv5rt;



pub fn initialize() {
    runtime::initialize();
}