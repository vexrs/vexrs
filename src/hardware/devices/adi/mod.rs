use alloc::vec::Vec;

use crate::{hardware::util::get_device_manager, runtime::get_runtime};

use super::{Device, ADIPort, ADIDevice};



// ADI encoder
pub mod encoder;








/// Sets the port type of an adi port
fn set_adi_config(device: vexv5rt::V5_DeviceT, port: u32,  port_type: ADIPort) {
    unsafe {
        vexv5rt::vexDeviceAdiPortConfigSet(device, port, port_type as u32);
    }
}

/// Gets the port type of an adi port
/// # Safety
/// This function may dereference a null pointer. It is the responsibility of the caller to ensure that the device is valid
/// and that the port is in range 0-7
fn get_adi_config(device: vexv5rt::V5_DeviceT, port: u32) -> ADIPort {
    unsafe {
        let port_type = vexv5rt::vexDeviceAdiPortConfigGet(device, port);
        ADIPort::from_u32(port_type)   
    }
}

/// Sets the value of an ADI port
fn set_adi_value(device: vexv5rt::V5_DeviceT, port: u32, value: i32) {
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

        // If the port is not a digital in, panic
        if get_adi_config(self.get_vex_device(0), self.index) != ADIPort::DigitalIn {
            panic!("Port {} is not a digital in port", self.index);
        }

        // Lock the mutex for the port
        let _mtx = get_device_manager().lock_adi_device(self.port, self.index, ADIPort::DigitalIn);

        // Read the value
        let value = get_adi_value(self.get_vex_device(0), self.index);

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
        // Set the port type
        set_adi_config(self.get_vex_device(0), self.index, ADIPort::DigitalIn);
    }

    fn calibrate(&mut self) {
        // Raw digital in ports do not need calibration
    }

    fn get_smart_ports(&self) -> Vec<(u32, super::SmartPort)> {
        // Get the device manager
        let dm = crate::util::get_device_manager();
    
        // Get the smart port
        let port = dm.get_port(self.port);

        // Return the smart port
        vec![(self.port, port)]
    }

    fn get_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl ADIDevice for ADIDigitalIn {
    fn new_adi(ports: Vec<(u32, u32)>) -> Self {
        // Get the first port
        let (port, index) = ports[0];

        // Create a new ADI digital in device
        ADIDigitalIn::new(port, index)
    }

    fn get_adi_ports(&self) -> Vec<(u32, u32, ADIPort)> {
        // Return the port
        vec![(self.port, self.index, ADIPort::DigitalIn)]
    }
}



/// A basic ADI digital out device
#[derive(Copy, Clone)]
pub struct ADIDigitalOut {
    /// The port number of this device
    port: u32,
    /// The ADI port that this device is connected to
    index: u32,
}

impl ADIDigitalOut {
    /// Creates a new ADI digital out device
    pub fn new(port: u32, index: u32) -> Self {
        ADIDigitalOut { port, index }
    }

    /// Writes to the ADI digital out device
    pub fn write(&self, value: bool) {

        // If the port is not a digital out, panic
        if get_adi_config(self.get_vex_device(0), self.index) != ADIPort::DigitalOut {
            panic!("Port {} is not a digital out port", self.index);
        }

        // Lock the mutex for the port
        let _mtx = get_device_manager().lock_adi_device(self.port, self.index, ADIPort::DigitalOut);

        // Write the value
        set_adi_value(self.get_vex_device(0), self.index, if value { 1 } else { 0 });
    }
}

impl Device for ADIDigitalOut {
    fn init(&mut self) {
        // Set the port type
        set_adi_config(self.get_vex_device(0), self.index, ADIPort::DigitalOut);
    }

    fn calibrate(&mut self) {
        // Raw digital out ports do not need calibration
    }

    fn get_smart_ports(&self) -> Vec<(u32, super::SmartPort)> {
        // Get the device manager
        let dm = crate::util::get_device_manager();
    
        // Get the smart port
        let port = dm.get_port(self.port);

        // Return the smart port
        vec![(self.port, port)]
    }

    fn get_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl ADIDevice for ADIDigitalOut {
    fn new_adi(ports: Vec<(u32, u32)>) -> Self {
        // Get the first port
        let (port, index) = ports[0];

        // Create a new ADI digital out device
        ADIDigitalOut::new(port, index)
    }

    fn get_adi_ports(&self) -> Vec<(u32, u32, ADIPort)> {
        // Return the port
        vec![(self.port, self.index, ADIPort::DigitalOut)]
    }
}



/// A basic ADI analog in device
#[derive(Copy, Clone)]
pub struct ADIAnalogIn {
    /// The port number of this device
    port: u32,
    /// The ADI port that this device is connected to
    index: u32,
}

impl ADIAnalogIn {
    /// Creates a new ADI analog in device
    pub fn new(port: u32, index: u32) -> Self {
        ADIAnalogIn { port, index }
    }

    /// Reads from the ADI analog in device
    pub fn read(&self) -> i32 {

        // If the port is not a analog in, panic
        if get_adi_config(self.get_vex_device(0), self.index) != ADIPort::AnalogIn {
            panic!("Port {} is not a analog in port", self.index);
        }

        // Lock the mutex for the port
        let _mtx = get_device_manager().lock_adi_device(self.port, self.index, ADIPort::AnalogIn);

        // Read the value
        get_adi_value(self.get_vex_device(0), self.index)
    }
}

impl Device for ADIAnalogIn {
    fn init(&mut self) {
        // Set the port type
        set_adi_config(self.get_vex_device(0), self.index, ADIPort::AnalogIn);
    }

    fn calibrate(&mut self) {
        // Raw analog in ports do not need calibration
    }

    fn get_smart_ports(&self) -> Vec<(u32, super::SmartPort)> {
        // Get the device manager
        let dm = crate::util::get_device_manager();
    
        // Get the smart port
        let port = dm.get_port(self.port);

        // Return the smart port
        vec![(self.port, port)]
    }

    fn get_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl ADIDevice for ADIAnalogIn {
    fn new_adi(ports: Vec<(u32, u32)>) -> Self {
        // Get the first port
        let (port, index) = ports[0];

        // Create a new ADI analog in device
        ADIAnalogIn::new(port, index)
    }

    fn get_adi_ports(&self) -> Vec<(u32, u32, ADIPort)> {
        // Return the port
        vec![(self.port, self.index, ADIPort::AnalogIn)]
    }
}



/// A basic ADI analog out device
#[derive(Copy, Clone)]
pub struct ADIAnalogOut {
    /// The port number of this device
    port: u32,
    /// The ADI port that this device is connected to
    index: u32,
}

impl ADIAnalogOut {
    /// Creates a new ADI analog out device
    pub fn new(port: u32, index: u32) -> Self {
        ADIAnalogOut { port, index }
    }

    /// Writes to the ADI analog out device
    pub fn write(&self, value: i32) {

        // If the port is not a analog out, panic
        if get_adi_config(self.get_vex_device(0), self.index) != ADIPort::AnalogOut {
            panic!("Port {} is not a analog out port", self.index);
        }

        // Lock the mutex for the port
        let _mtx = get_device_manager().lock_adi_device(self.port, self.index, ADIPort::AnalogOut);

        // Write the value
        set_adi_value(self.get_vex_device(0), self.index, value);
    }
}

impl Device for ADIAnalogOut {
    fn init(&mut self) {
        // Set the port type
        set_adi_config(self.get_vex_device(0), self.index, ADIPort::AnalogOut);
    }

    fn calibrate(&mut self) {
        // Raw analog out ports do not need calibration
    }

    fn get_smart_ports(&self) -> Vec<(u32, super::SmartPort)> {
        // Get the device manager
        let dm = crate::util::get_device_manager();
    
        // Get the smart port
        let port = dm.get_port(self.port);

        // Return the smart port
        vec![(self.port, port)]
    }

    fn get_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl ADIDevice for ADIAnalogOut {
    fn new_adi(ports: Vec<(u32, u32)>) -> Self {
        // Get the first port
        let (port, index) = ports[0];

        // Create a new ADI analog out device
        ADIAnalogOut::new(port, index)
    }

    fn get_adi_ports(&self) -> Vec<(u32, u32, ADIPort)> {
        // Return the port
        vec![(self.port, self.index, ADIPort::AnalogOut)]
    }
}