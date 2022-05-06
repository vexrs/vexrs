// A structure that manages the state of all devices connected to the brain

use core::borrow::Borrow;

use alloc::{vec::Vec, boxed::Box};

use crate::runtime::mutex::Mutex;

use super::{SmartPort, Device};




/// Manages the persistent state of all devices connected to the brain
pub struct DeviceManager {
    /// All the smart ports connected to the V5 brain.
    /// Note that each smart port does not mean a single device.
    /// For example, ADI expanders can have 8 ADI devices, and
    /// motors have both encoders and motors.
    smart_ports: [Mutex<SmartPort>; 24],
    /// This is a vector of all devices on the robot brain.
    /// Devices can only be added to this vector if it is confirmed that their
    /// smart port is not occupied. Once it is comfirmed, their port is reserved.
    devices: Vec<Mutex<Box<dyn Device>>>,
}

impl DeviceManager {

    /// Creates a new device manager
    pub fn new() -> Self {
        Self {
            smart_ports: Default::default(),
            devices: Vec::new(),
        }
    }

    /// Initializes the device manager

    /// Gets a copy of the smart port at the given index
    pub fn get_port(&self, index: u32) -> SmartPort {
        // Acquire the mutex on the smart port
        let mtx = self.smart_ports[index as usize].acquire();

        // Return the smart port info.
        *mtx
    }
}