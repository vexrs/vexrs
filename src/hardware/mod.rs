
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

// The global device manager singleton.
static mut DEVICE_MANAGER: *const devices::manager::DeviceManager = 0 as *const devices::manager::DeviceManager;


// Utilities
pub mod util;


/// The global display singleton
static mut DISPLAY: *const display::Display = 0 as *const display::Display;