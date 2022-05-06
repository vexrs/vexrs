

// The device manager
pub mod manager;

// ADI port implementations
pub mod adi;

/// Types of ADI ports
#[repr(u8)]
pub enum ADIPort {
    /// Analog in is for sensors such as the potentiometer or gyroscope
    AnalogIn = 0,
    /// Analog out is for anythign that recieves an analog signal, such as the distance sensor
    AnalogOut = 1,
    /// Digital in is for sensors such as the button or the limit switch
    DigitalIn = 2,
    /// Digital out is for anything that recieves a digital on/off signal.
    DigitalOut = 3,
    /// There is no sensor connected to this port
    None = 0xff,
}

impl ADIPort {
    /// Converts a u32 to an ADIPort
    pub fn from_u32(value: u32) -> ADIPort {
        match value {
            0 => ADIPort::AnalogIn,
            1 => ADIPort::AnalogOut,
            2 => ADIPort::DigitalIn,
            3 => ADIPort::DigitalOut,
            _ => ADIPort::None,
        }
    }
}

/// A Type of device connected to a smart port
pub enum SmartPort {
    /// No device is connected to this smart port
    None,
    /// An ADI expander is connected to this smart port
    /// ADI expanders can also have their ports reserved
    /// so they have 8x ADI Ports.
    ADIExpander([ADIPort; 8]),
}


/// This is an enum that allows us to nicely provide a way to get the underlying struct
/// that implements the device trait.
pub enum DeviceType {
    /// This device is an unknown device that we know is plugged in.
    EmptyDevice,
    ADIDigitalIn,
}


/// The device trait provides a common interface that all devices need to honor
/// including initialization, calibration, and port verification.
/// This is the base trait for all devices.
pub trait Device {
    /// Gets the type of the device, including the struct that implements this trait
    fn get_type(&self) -> DeviceType;

    /// Gets the vex device pointer
    fn get_vex_device(&self) -> vexv5rt::V5_DeviceT {

        // Get the smart port
        let port = self.get_port_number().0;

        // Subtract one from the port number to get the correct index
        let port = port - 1;

        // Ensure it is within the range 0-21
        if port < 0 || port > 21 {
            panic!("Port number is out of range");
        }

        // Get the pointer
        unsafe {
            vexv5rt::vexDeviceGetByIndex(port)
        }
    }

    /// Initializes the device
    fn init(&mut self);

    /// Calibrates the device
    fn calibrate(&mut self);

    /// Gets the port type of the device
    fn get_port_type(&self) -> (SmartPort, ADIPort);

    /// Gets the port number of the device
    /// including the smart port and the ADI port (in that order)
    /// Non ADI ports always return zero as the second tuple member.
    fn get_port_number(&self) -> (u8, u8);
}