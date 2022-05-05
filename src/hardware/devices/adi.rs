


/// An enum that represents ADI devices
#[derive(Default, Copy, Clone, PartialEq)]
pub enum ADIDevice {
    /// No device is attached to the given port
    #[default] None,
    
}


/// This structure creates ADI devices on a specified port
pub struct ADIBuilder<'a> {
    pub port: u8,
    pub device_manager: &'a mut super::DeviceManager
}

impl<'a> ADIBuilder<'a> {

    /// Creates a new raw ADI device on this port.
    pub fn create_raw(&self) {
        // If the current port on the device manager is not an ADI device,
        // then panic
        if self.device_manager.devices[self.port as usize].device_type != super::DeviceType::ADIRaw {
            panic!("Port {} is not an ADI device", self.port);
        }
    }
}