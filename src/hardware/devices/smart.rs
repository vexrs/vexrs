use super::{Device, SmartDevice};





/// A basic smart motor
#[derive(Clone)]
pub struct SmartMotor {
    /// The smart port that this motor is connected to
    port: u32,
}

impl SmartMotor {
    pub fn run(&self, speed: i32) {
        unsafe {
            vexv5rt::vexDeviceMotorVelocitySet(self.get_vex_device(0), speed);
        }
    }
}


impl Device for SmartMotor {
    fn init(&mut self) {
        // We do not need to do anything here
    }

    fn calibrate(&mut self) {
        // Nor here
    }

    fn get_smart_ports(&self) -> alloc::vec::Vec<(u32, super::SmartPort)> {
        vec![(self.port, super::SmartPort::Motor)]
    }

    fn get_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl SmartDevice for SmartMotor {
    fn new_smart(port: u32) -> Self {
        Self {
            port,
        }
    }

    fn get_smart_port(&self) -> u32 {
        self.port
    }

    fn get_smart_port_type(&self) -> super::SmartPort {
        super::SmartPort::Motor
    }
}