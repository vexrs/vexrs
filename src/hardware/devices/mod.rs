use alloc::vec::Vec;



// Module contains code for interfacing with ADI devices;
pub mod adi;




/// The type of a device connected to the brain
#[derive(Default, Copy, Clone, PartialEq)]
pub enum DeviceType {
    /// No device is attached to the given port
    #[default] None,
    /// An ADI expander is attached to the port
    ADIExpander,
}

/// The interface that a device is interacted with through
#[derive(Default, Copy, Clone, PartialEq)]
pub enum DeviceInterface {
    /// No device is attached to the given port or the
    /// device can not be interacted with
    #[default] None,
}


/// A device as connected to a V5 port.
#[derive(Default, Copy, Clone)]
pub struct Device {
    /// The type of the device
    device_type: DeviceType,
    /// The interface being used with the device
    device_interface: DeviceInterface,
    /// The vex device pointer
    device: vexv5rt::V5_DeviceT,
}


/// The device manager struct is used to manage user code adding devices and to store callibration state.
pub struct DeviceManager {
    devices: [Device; 24],
    adi_devices: Vec<adi::ADIDevice>,
}

impl DeviceManager {
    /// Creates a new device manager
    pub fn new() -> DeviceManager {
        DeviceManager { devices: [Device::default(); 24], adi_devices: Vec::new() }
    }

    /// Creates a default device manager instance
    pub fn default() -> DeviceManager {
        DeviceManager::new()
    }

    /// Sets a device on a specific port
    /// Returning the device if creation is successful or None if the port is already in use
    pub fn add_device(&mut self, port: u8, device_type: DeviceType, device_interface: DeviceInterface) -> Option<Device> {
        
        // Subtract one from the port to make it zero-indexed
        let port = port - 1;

        // If it is more than 21, then return None
        if port > 21 {
            return None;
        }

        // Create the device struct
        let device = Device {
            device_type,
            device_interface,
            device: unsafe { vexv5rt::vexDeviceGetByIndex(port as i32) }
        };

        // Set the device if it does not exist
        // and return existing if it is the same
        if self.devices[port as usize].device_type == DeviceType::None {
            self.devices[port as usize] = device;
            Some(device)
        } else if self.devices[port as usize].device_type == device.device_type {
            Some(self.devices[port as usize])
        } else {
            None
        }
    }

    /// Adds an ADI device on a specific port
    pub fn add_adi(&mut self, port: u8, index: u8, device: adi::ADIDevice))

    /// Creates an ADI expander on a specified port
    /// ADI expanders actually have no interface
    /// so this returns an ADI port builder.
    pub fn adi_expander(&mut self, port: u8) -> adi::ADIBuilder {

        // Create the ADI expander device
        let device = Device {
            device_type: DeviceType::ADIExpander,
            device_interface: DeviceInterface::None,

        };

        // Add it to the device manager
        let device = self.add_device(port, device);

        // If it failed, panic
        if device.is_none() {
            panic!("Failed to add ADI expander to port {}", port);
        }

        // Return an ADI builder that allows ADI ports to be created on this port
        adi::ADIBuilder {
            port: port,
            device_manager: self,
        }
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