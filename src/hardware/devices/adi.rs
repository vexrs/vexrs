use super::{Device, SmartPort, ADIPort, DeviceType, manager::DeviceManager};



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
pub struct ADIDigitalIn {
    /// The port number of this device
    port_number: u32,
    /// The ADI port that this device is connected to
    port: u32,
    /// The device manager that owns this device
    device_manager: &'static mut DeviceManager,
}


impl ADIDigitalIn {
    /// Creates a new ADI digital in device
    pub fn new(port_number: u32, port: u32) -> ADIDigitalIn {
        let device_manager: Option<&'static mut DeviceManager> = crate::util::get_device_manager();
        ADIDigitalIn {
            port_number,
            port,
            device_manager: device_manager.unwrap(),
        }
    }

    /// Reads the value of the digital in device
    pub fn read(&self) -> i32 {
        get_adi_value(self.get_vex_device(), self.port)
    }
}


impl Device for ADIDigitalIn {
    fn get_type(&self) -> DeviceType {
        DeviceType::ADIDigitalIn
    }

    fn init(&mut self) {
        // Configure the port to be digital in
        set_adi_config(self.get_vex_device(), self.port, ADIPort::DigitalIn);
    }

    fn calibrate(&mut self) {
        // Raw digital ports do not need to be calibrated
    }

    fn get_port_type(&self) -> (SmartPort, ADIPort) {
        (self.device_manager.get_port(self.port_number), ADIPort::DigitalIn)
    }

    fn get_port_number(&self) -> (u32, u32) {
        (self.port_number, self.port)
    }
}

