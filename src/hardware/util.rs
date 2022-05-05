use super::{display::Display, devices::DeviceManager};


/// Gets the global display
pub fn get_display() -> &'static mut Display {
    unsafe {
        let disp = super::DISPLAY as *mut Display;
        &mut *disp
    }
}

/// Gets the global device manager
pub fn get_device_manager() -> &'static mut DeviceManager {
    unsafe {
        let disp = super::DEVICE_MANAGER as *mut DeviceManager;
        &mut *disp
    }
}