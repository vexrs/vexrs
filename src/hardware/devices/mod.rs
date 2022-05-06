

// The device manager
pub mod manager;

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
    ADIDigitalIn,
}


/// The device trait provides a common interface that all devices need to honor
/// including initialization, calibration, and port verification.
/// This is the base trait for all devices.
pub trait Device {
    /// Gets the type of the device, including the struct that implements this trait
    fn get_type(&self) -> DeviceType;

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