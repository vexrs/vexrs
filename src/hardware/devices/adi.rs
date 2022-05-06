use crate::{hardware::util::get_device_manager, runtime::{mutex::Mutex, get_runtime}};

use super::{Device, SmartPort, ADIPort, DeviceType, manager::DeviceManager, ADIDevice};



/// Sets the port type of an adi port
fn set_adi_config(device: vexv5rt::V5_DeviceT, port: u32,  port_type: ADIPort) {
    unsafe {
        vexv5rt::vexDeviceAdiPortConfigSet(device, port, port_type as u32);
    }
}

/// Gets the port type of an adi port
fn get_adi_config(device: vexv5rt::V5_DeviceT, port: u32) -> ADIPort {
    unsafe {
        let port_type = vexv5rt::vexDeviceAdiPortConfigGet(device, port);
        ADIPort::from_u32(port_type)
    }
}

/// Sets the value of an ADI port
fn g(device: vexv5rt::V5_DeviceT, port: u32, value: i32) {
    unsafe {
        vexv5rt::vexDeviceAdiValueSet(device, port, value);
    }
}

/// Gets the value of an ADI port
fn get_adi_value(device: vexv5rt::V5_DeviceT, port: u32) -> i32 {
    unsafe {
        vexv5rt::vexDeviceAdiValueGet(device, port)
    }
}


/// A basic ADI digital in device
#[derive(Copy, Clone)]
pub struct ADIDigitalIn {
    /// The port number of this device
    port: u32,
    /// The ADI port that this device is connected to
    index: u32,
    /// The previously read value
    last_value: bool,
}

impl ADIDigitalIn {
    /// Creates a new ADI digital in device
    pub fn new(port: u32, index: u32) -> Self {
        ADIDigitalIn {
            port,
            index,
            last_value: false,
        }
    }

    /// Reads from the ADI digital in device
    pub fn read(&self) -> bool {
        // Lock the mutex for the port
        let _mtx = get_device_manager().unwrap().lock_adi_device(self.port, self.index, ADIPort::DigitalIn);

        // Read the value
        let value = get_adi_value(self.get_vex_device(), self.index);

        // Return it
        value != 0
    }

    /// Returns true if the digital signal went high since the previous check
    pub fn on_rising_edge(&mut self) -> bool {

        // Get the new value
        let new_value = self.read();

        // If the values are different and the new value is true, the digital signal 
        // went high since the previous check
        if (self.last_value != new_value) && new_value {
            // Set it in the cache and return true
            self.last_value = new_value;
            true
        } else {
            // Otherwise, return false
            false
        }

    }

    /// Returns true if the digital signal went low since the previous check
    pub fn on_falling_edge(&mut self) -> bool {

        // Get the new value
        let new_value = self.read();

        // If the values are different and the new value is false, the digital signal 
        // went low since the previous check
        if (self.last_value != new_value) && !new_value {
            // Set it in the cache and return true
            self.last_value = new_value;
            true
        } else {
            // Otherwise, return false
            false
        }

    }


    /// Blocks until the digital signal goes high
    pub fn await_high(&mut self) {
        // Get the runtime
        let runtime = get_runtime();

        // yield for as long as the value is false
        while !self.read() {
            runtime.yield_t();
        }
    }

    /// Blocks until the digital signal goes low
    pub fn await_low(&mut self) {
        // Get the runtime
        let runtime = get_runtime();

        // yield for as long as the value is true
        while self.read() {
            runtime.yield_t();
        }
    }

    
}

impl Device for ADIDigitalIn {

    fn init(&mut self) {
        // Configure the port to be digital in
        set_adi_config(self.get_vex_device(), self.index, ADIPort::DigitalIn);
    }

    fn calibrate(&mut self) {
        // Raw digital ports do not need to be calibrated
    }


    fn get_any(&self) -> &dyn core::any::Any {
        self
    }

    fn get_port_number(&self) -> u32 {
        self.port
    }
}

impl ADIDevice for ADIDigitalIn {
    fn new_adi(port: u32, index: u32) -> Self {
        ADIDigitalIn { port, index, last_value: false }
    }

    fn get_adi_port(&self) -> ADIPort {
        ADIPort::DigitalIn
    }
}

