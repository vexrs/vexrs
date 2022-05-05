

/// An ADI digital in port for sensors such as the bumpers or limit switches.
#[derive(Copy, Clone, PartialEq)]
pub struct ADIDigitalIn {
    pub port: u8,
    pub device_no: u8,
}

impl ADIDigitalIn {
    /// Reads the value from the digital in port
    pub fn read(&self) -> bool {
        true
    }
}

impl super::Device for ADIDigitalIn {
    fn is_calibrated(&self) -> bool {
        true
    }

    fn calibrate(&mut self) {
        
    }

    fn get_ports(&self) -> (u8, u8) {
        (self.port, self.device_no)
    }

    fn get_type(&self) -> super::DeviceType {
        super::DeviceType::ADIDigitalIn
    }
}