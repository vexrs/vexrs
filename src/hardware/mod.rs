
// Basic timer implementation
pub mod timer;

// Serial interaction library
pub mod serial;

// Controller implementation
pub mod controller;

// Utilities for getting the competition status
pub mod competition;

// Basic hardware abstraction on top of the v5 brain.
#[cfg(feature="graphics")]
pub mod display;

// Utilities
pub mod util;


/// The global display singleton
static mut DISPLAY: *const display::Display = 0 as *const display::Display;