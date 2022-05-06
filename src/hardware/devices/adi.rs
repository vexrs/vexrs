use super::{Device, SmartPort, ADIPort, DeviceType};



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
    /// The smart port that this device is connected to
    smart_port: SmartPort,
    /// The port number of this device
    port_number: u32,
    /// The ADI port that this device is connected to
    port: u32,
}


impl ADIDigitalIn {
    /// Creates a new ADI digital in device
    pub fn new(smart_port: SmartPort, port_number: u32, port: u32) -> ADIDigitalIn {
        ADIDigitalIn {
            smart_port,
            port_number,
            port,
        }
    }

    /// Reads the value of the digital in device
    pub fn read(&self) -> i32 {
        unsafe {
            vexv5rt::vexDeviceAdiValueGet(self.get_vex_device(), self.port)
        }
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
        (self.smart_port, ADIPort::DigitalIn)
    }

    fn get_port_number(&self) -> (u8, u8) {
        (self.port_number, self.port)
    }
}

