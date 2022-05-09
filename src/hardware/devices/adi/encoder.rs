use alloc::vec::Vec;
use crate::hardware::{devices::{ADIPort, Device, ADIDevice, Encoder}, util::get_device_manager};

use super::{set_adi_config, set_adi_value, get_adi_value};





/// ADI encoder device
/// Takes two ports and indexes
#[derive(Clone)]
pub struct ADIEncoder {
    ports: Vec<(u32, u32, ADIPort)>,
}

impl ADIEncoder {
    /// Reads the data from the encoder ports
    fn read_encoder(&self) -> (i32, i32) {

        // Lock the device
        let _mtx = self.lock();

        // Read from the first port
        let v1 = get_adi_value(self.get_vex_device(0), self.ports[0].1);

        // Read from the second port
        let v2 = get_adi_value(self.get_vex_device(1), self.ports[1].1);
    
        // Return the values
        (v1,v2)
    }
}


impl Device for ADIEncoder {
    fn init(&mut self) {

        // Lock the device
        let _mtx = self.lock();

        // Set both ports as an encoder
        set_adi_config(self.get_vex_device(0), self.ports[0].1, ADIPort::QuadEncoder);
        set_adi_config(self.get_vex_device(1), self.ports[1].1, ADIPort::QuadEncoder);
    }

    fn calibrate(&mut self) {
        // The brain takes care of this for us.
        // Just reset the encoders back to zero
        self.reset_encoder();
    }

    fn get_smart_ports(&self) -> Vec<(u32, crate::hardware::devices::SmartPort)> {
        // Lock the device
        let _mtx = self.lock();

        // Get the smart ports that we use
        let mut ports = Vec::new();
        for port in self.ports.iter() {
            ports.push((port.0, get_device_manager().get_port(port.0)));
        }
        ports
    }

    fn get_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl ADIDevice for ADIEncoder {
    fn new_adi(ports: Vec<(u32, u32)>) -> Self {
        // If there are less than two elements, panic
        if ports.len() < 2 {
            panic!("ADIEncoder requires at least two ports");
        }
        // Create a new ADI encoder
        Self {
            ports: ports
                .into_iter()
                .map(|(port, index)| (port, index, ADIPort::QuadEncoder))
                .collect(),
        }
    }

    fn get_adi_ports(&self) -> Vec<(u32, u32, ADIPort)> {
        self.ports.clone()
    }
}

impl Encoder for ADIEncoder {
    fn get_ticks(&self) -> f64 {

        self.read_encoder().0.into()
    }

    fn get_rate(&self) -> f64 {
        self.read_encoder().1.into()
    }

    fn reset_encoder(&mut self) {

        // Lock the device
        let _mtx = self.lock();

        set_adi_value(self.get_vex_device(0), self.ports[0].1, 0);
        set_adi_value(self.get_vex_device(1), self.ports[1].1, 0);
    }

    fn set_zero_position(&mut self, position: f64) {
        // Lock the device
        let _mtx = self.lock();

        set_adi_value(self.get_vex_device(0), self.ports[0].1, (position as i64).try_into().unwrap());
        set_adi_value(self.get_vex_device(1), self.ports[1].1, (position as i64).try_into().unwrap());
    }

    
}