use crate::runtime::mutex::MutexGuard;

use super::{display::Display, devices::manager::DeviceManager};


/// Gets the global display
pub fn get_display<'a>() -> MutexGuard<'a, Display> {
    crate::DISPLAY.acquire()
}

/// Gets the global device manager
pub fn get_device_manager<'a>() -> MutexGuard<'a, DeviceManager> {
    crate::DEVICE_MANAGER.acquire()
}