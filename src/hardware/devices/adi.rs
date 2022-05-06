


/// Basic ADI digital in port
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ADIDigitalIn {
    /// The ADI expander port number
    port: u8,
    /// The ADI device number
    index: u8,
}

impl ADIDigitalIn {
    /// Creates a new ADI digital in port
    pub fn new(port: u8, index: u8) -> ADIDigitalIn {
        ADIDigitalIn { port, index }
    }

    /// Gets the port number
    pub fn port(&self) -> u8 {
        self.port
    }

    /// Gets the device number
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Reads the value of the port
    pub fn read(&self) ->i32 {
       unsafe {
           vexv5rt::vexAdiValueGet(self.port.into(), self.index.into())
       }
    }
}

/// An enum that represents ADI device types
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum ADIDevice {
    /// No device is attached to the given port
    #[default] None,
    /// This is an ADI digital in port
    DigitalIn(ADIDigitalIn),
}

/// This structure creates ADI devices on a specified port
pub struct ADIBuilder<'a> {
    pub port: u8,
    pub device_manager: &'a mut super::DeviceManager
}

impl<'a> ADIBuilder<'a> {

    /// Creates a new digital in device
    pub fn digital_in(&mut self, index: u8) -> ADIDigitalIn {

        // Subtract 1 from the index to make it zero indexed
        let index = index - 1;

        // Create the digital in device
        let din = ADIDigitalIn::new(self.port, index);
        
        // Add the device to the device manager
        let device = self.device_manager.add_adi(self.port, index, ADIDevice::DigitalIn(din));
        
        // If it failed, panic
        if device.is_none() {
            panic!("Failed to add ADI digital in device {}:{}", self.port, index);
        }

        // Return the device
        din
    }

}