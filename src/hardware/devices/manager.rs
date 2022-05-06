// A structure that manages the state of all devices connected to the brain

use alloc::{vec::Vec, boxed::Box};

use crate::runtime::mutex::{Mutex, MutexGuard};

use super::{SmartPort, Device, ADIPort, ADIDevice, SmartDevice};




/// Manages the persistent state of all devices connected to the brain
pub struct DeviceManager {
    /// All the smart ports connected to the V5 brain.
    /// Note that each smart port does not mean a single device.
    /// For example, ADI expanders can have 8 ADI devices, and
    /// motors have both encoders and motors.
    pub smart_ports: [Mutex<SmartPort>; 22],
    /// A vector of the mutex locks for all ADI devices
    pub adi_ports: Vec<(u32,u32, Mutex<()>)>,
    /// This is a vector of all devices on the robot brain.
    /// Devices can only be added to this vector if it is confirmed that their
    /// smart port is not occupied. Once it is comfirmed, their port is reserved.
    pub devices: Vec<Box<dyn Device>>,
}

impl Default for DeviceManager {
    /// Creates a default device manager
    fn default() -> Self {
        DeviceManager::new()
    }
}

impl DeviceManager {

    /// Creates a new device manager
    pub fn new() -> Self {
        Self {
            smart_ports: Default::default(),
            adi_ports: Default::default(),
            devices: Vec::new(),
        }
    }

    /// Initializes the device manager
    pub fn init(&mut self) {
        // Set the global device manager
        unsafe {
            crate::hardware::DEVICE_MANAGER = self as *const DeviceManager;
        }

        // Initialize smart port 22 as the built-in ADI expander
        self.adi_expander(21);
    }

    /// Reserves a smart port
    pub fn reserve_port(&mut self, port: u32, device: SmartPort) {

        // Bounds check the port
        if port > 21{
            panic!("Port {} is out of bounds", port);
        }
        
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

    /// Reserves a port for an adi device
    fn reserve_adi(&mut self, port: u32, index: u32, device: ADIPort) {

        // Bounds check the port and index
        if port > 21 || index > 7 {
            panic!("ADI port {}:{} is out of bounds", port, index);
        }


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

    /// Locks the mutex of an ADI port
    pub fn lock_adi_device(&mut self, port: u32, index: u32, adi_type: ADIPort) -> MutexGuard<()> {

        // Bounds check the port and index
        if port > 21 || index > 7 {
            panic!("ADI port {}:{} is out of bounds", port, index);
        }



        // Verify that the port is an ADI expander
        match *self.smart_ports[port as usize].acquire() {
            SmartPort::ADIExpander(ref mut adi_port) => {
                // Verify that the port matches the ADI type
                if adi_port[index as usize] != adi_type {
                    panic!("ADI port {}:{} is not a {:?}", port, index, adi_type);
                }
            },
            _ => panic!("Smart port {} is not an ADI expander", port),
        };

        // Check if this port exists in the list of devices
        let mut found = false;
        let mut loc = 0;
        for (i, device) in self.adi_ports.iter().enumerate() {
            if device.0 == port && device.1 == index {
                found = true;
                loc = i;
                break;
            }
        }

        // If it does not, then add it (we already know it *should* be there because we verified above)
        if !found {
            self.adi_ports.push((port, index, Mutex::new(())));
            loc = self.adi_ports.len() - 1;
        }

        // Lock the mutex and return the guard
        self.adi_ports[loc as usize].2.acquire()
    }



    /*
    /*****************************************************\
    |**********  User Accesible Utilities  ***************|
    \*****************************************************/
    */

    /// Reserves a port for an adi expander
    pub fn adi_expander(&mut self, port: u32) {

        // Reserve the port
        self.reserve_port(port, SmartPort::ADIExpander(Default::default()));
    }

    /// Gets or sets up a ADI port
    pub fn get_adi_device<T: 'static + ADIDevice + Clone>(&mut self, ports: Vec<(u32, u32)>) -> T {
        // Bounds check the ports and indexes
        for (port, index) in ports.iter() {
            if *port > 21 || *index > 7 {
                panic!("ADI port {}:{} is out of bounds", port, index);
            }
        }

        // Create the new device
        let mut device = T::new_adi(ports);

        // Reserve all required ADI ports
        for p in device.get_adi_ports() {
            self.reserve_adi(p.0, p.1, p.2);
        }
        

        // Initialize the device
        device.init();

        // Add the device to the list of devices
        self.devices.push(Box::new(device.clone()));

        device
    }

    /// Gets or sets up a smart device
    pub fn get_smart_device<T: 'static + SmartDevice + Clone>(&mut self, port: u32) -> T {

        // Bounds check the port
        if port > 21 {
            panic!("Port {} is out of bounds", port);
        }

        // Create the new device
        let mut device = T::new_smart(port);

        // Reserve the port
        self.reserve_port(port, device.get_smart_port_type());

        // Initialize the device
        device.init();

        // Add the device to the list of devices
        self.devices.push(Box::new(device.clone()));

        device
    }

    /// Gets a copy of the smart port at the given index
    pub fn get_port(&self, index: u32) -> SmartPort {
        // Acquire the mutex on the smart port
        let mtx = self.smart_ports[index as usize].acquire();

        // Return the smart port info.
        *mtx
    }
}