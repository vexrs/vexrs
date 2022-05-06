use crate::hardware::util::get_device_manager;

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
    port_number: u32,
    /// The ADI port that this device is connected to
    port: u32,
}

impl ADIDigitalIn {
    /// Creates a new ADI digital in device
    pub fn new(port_number: u32, port: u32) -> Self {
        ADIDigitalIn {
            port_number,
            port,
        }
    }

    /// Reads from the ADI digital in device
    pub fn read(&self) -> i32 {
        // Lock the mutex for the port
        let _mtx = get_device_manager().unwrap().lock_adi_device(self.port_number, self.port, ADIPort::DigitalIn);

        // Read the value
        let value = get_adi_value(self.get_vex_device(), self.port);

        // Return it
        value
    }
}

impl Device for ADIDigitalIn {

    fn init(&mut self) {
        // Configure the port to be digital in
        set_adi_config(self.get_vex_device(), self.port, ADIPort::DigitalIn);
    }

    fn calibrate(&mut self) {
        // Raw digital ports do not need to be calibrated
    }


    fn get_any(&self) -> &dyn core::any::Any {
        self
    }

    fn get_port_number(&self) -> u32 {
        self.port_number
    }
}


impl ADIDevice for ADIDigitalIn {
    fn new_adi(port: u32, index: u32) -> Self {
        ADIDigitalIn { port_number: port, port: index }
    }

    fn get_adi_port(&self) -> ADIPort {
        ADIPort::DigitalIn
    }
}