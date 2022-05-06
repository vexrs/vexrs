use super::{display::Display, devices::manager::DeviceManager};


/// Gets the global display
/// Returns None if the display is not initialized
pub fn get_display() -> Option<&'static mut Display> {
    unsafe {
        if super::DISPLAY.is_null() {
            None
        } else {
            let disp = super::DISPLAY as *mut Display;
            Some(&mut *disp)
        }
    }
}

/// Gets the global device manager
/// Returns None if the device manager has not been initialized
pub fn get_device_manager() -> Option<&'static mut DeviceManager> {
    unsafe {
        if super::DEVICE_MANAGER.is_null() {
            None
        } else {
            let disp = super::DEVICE_MANAGER as *mut DeviceManager;
            Some(&mut *disp)
        }
    }
}