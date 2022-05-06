// A structure that manages the state of all devices connected to the brain

use alloc::{vec::Vec, boxed::Box};

use crate::runtime::mutex::{Mutex, MutexGuard};

use super::{SmartPort, Device, ADIPort, adi::ADIDigitalIn, DeviceType};




/// Manages the persistent state of all devices connected to the brain
pub struct DeviceManager {
    /// All the smart ports connected to the V5 brain.
    /// Note that each smart port does not mean a single device.
    /// For example, ADI expanders can have 8 ADI devices, and
    /// motors have both encoders and motors.
    pub smart_ports: [Mutex<SmartPort>; 22],
    /// This is a vector of all devices on the robot brain.
    /// Devices can only be added to this vector if it is confirmed that their
    /// smart port is not occupied. Once it is comfirmed, their port is reserved.
    pub devices: Vec<Box<dyn Device>>,
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
    pub fn init(&self) {
        // Set the global device manager
        unsafe {
            crate::hardware::DEVICE_MANAGER = self as *const DeviceManager;
        }
    }

    /// Reserves a smart port
    pub fn reserve_port(&mut self, port: u32, device: SmartPort) {

        // Subtract 1 from the port to make it zero-indexed
        let port = port - 1;
        
        // Lock the mutex for this device
        let mut sport = self.smart_ports[port as usize].acquire();

        

        // If the port is none, set it to the new device
        if *sport == SmartPort::None {
            *sport = device;
        } else {
            // If not, and the device is not the same, panic
            if *sport != device {
                panic!("Smart port {} is already occupied by a different device", port);
            }
        }
    }

    /// Reserves a port for an adi expander
    pub fn adi_expander(&mut self, port: u32) {
        // Reserve the port
        self.reserve_port(port, SmartPort::ADIExpander(Default::default()));
    }

    /// Reserves a port for an adi device
    pub fn reserve_adi(&mut self, port: u32, index: u32, device: ADIPort) {

        // Make sure port and index are both zero-indexed
        let port = port - 1;
        let index = index - 1;

        // Get the smart port
        let mut smart_port = self.smart_ports[port as usize].acquire();

        // Get the ADI port or panic if it is not an adi expander
        let adi_port = match *smart_port {
            SmartPort::ADIExpander(ref mut adi_port) => &mut adi_port[index as usize],
            _ => panic!("Smart port {} is not an ADI expander", port),
        };

        // If the port is none, set it to the new device
        if *adi_port == ADIPort::None {
            *adi_port = device;
        } else {
            // If not, and the device is not the same, panic
            if *adi_port != device {
                panic!("ADI port {} is already occupied by a different device", index);
            }
        }
    }

    /// Adds an ADI digital device
    pub fn adi_digital_in(&mut self, port: u32, index: u32) -> &Box<dyn Device> {
        
        // Make sure the port and device are of the proper type
        match *self.smart_ports[port as usize - 1].acquire() {
            SmartPort::ADIExpander(ref mut adi_port) => {
                match adi_port[index as usize - 1] {
                    ADIPort::DigitalIn => {},
                    _ => panic!("ADI port {}:{} is not a digital in", port, index),
                }
            },
            _ => panic!("Smart port {} is not an ADI expander", port),
        };

        // Create the device
        let device = ADIDigitalIn::new(port, index);

        // Add the device to the list of devices
        self.devices.push(Box::new(device));

        // Do some magic to get the device out of the vector and return it
        self.devices.last().unwrap()
    }

    /// Gets a copy of the smart port at the given index
    pub fn get_port(&self, index: u32) -> SmartPort {
        // Acquire the mutex on the smart port
        let mtx = self.smart_ports[index as usize].acquire();

        // Return the smart port info.
        *mtx
    }
}