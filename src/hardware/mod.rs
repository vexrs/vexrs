
// Basic timer implementation
pub mod timer;

// Serial interaction library
pub mod serial;

// Controller implementation
pub mod controller;

// Utilities for getting the competition status
pub mod competition;

// Basic hardware abstraction on top of the v5 brain display.
#[cfg(feature="graphics")]
pub mod display;

// Structures to manage devices plugged into smart and three-wire ports.
pub mod devices;

// Utilities
pub mod util;