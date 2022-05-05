use core::{borrow::Borrow, cell::RefCell};

use alloc::{vec::Vec, boxed::Box, sync::Arc, rc::Rc};


/// ADI devices
pub mod adi;

/// The type of a device
#[derive(Clone, Copy, PartialEq)]
pub enum DeviceType {
    /// No device on this port
    None,
    /// An ADI digital in port
    ADIDigitalIn,
}

/// A device on a V5 brain
pub trait Device {
    /// Returns true if the device is calibrated
    fn is_calibrated(&self) -> bool;

    /// Calibrates the device
    fn calibrate(&mut self);

    /// Gets the ports if the device.
    /// First item will be the smart port, and second one will
    /// be the ADI port if it is a three wire device
    fn get_ports(&self) -> (u8, u8);

    /// Gets the type of the port
    fn get_type(&self) -> DeviceType;
}

/// The device manager struct is used to manage user code adding devices and to store callibration state.
pub struct DeviceManager {
    devices: Vec<Rc<RefCell<dyn Device>>>
}

impl DeviceManager {
    /// Creates a new device manager
    pub fn new() -> DeviceManager {
        DeviceManager { devices: Vec::new() }
    }

    /// Adds a new device to the manager
    /// if the port is free. If not and the new device
    /// has the same type as the existing device
    /// it will return the already existing device.
    /// Otherwise will panic
    pub fn add_device(&mut self, device: Rc<RefCell<dyn Device>>) -> Rc<RefCell<dyn Device>> {


        // Iterate over all devices
        for d in &self.devices {
            // Borrow the device
            let d_borrow = d.borrow_mut();

            // If the devices have the same port
            if d_borrow.borrow().get_ports() == device.borrow_mut().get_ports() {
                // If they have the same type, then return the original device
                if d_borrow.get_type() == device.borrow_mut().get_type() {
                    return Rc::clone(d);
                } else {
                    // If not, then panic
                    panic!("Port {}:{} can not be reused", d_borrow.get_ports().0, d_borrow.get_ports().1);
                }
            }
        }

        // Add the device
        self.devices.push(Rc::clone(&device));

        device
    }


    /// Initializes the device manager
    /// setting the global singleton'
    pub fn init(&self) {
        unsafe {
            // Set the global device manager
            super::DEVICE_MANAGER = self as *const DeviceManager;
        }
    }
    
}