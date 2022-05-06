use super::{Device, SmartDevice, Encoder};



/// The units to use in an encoder tick
#[derive(Default, Copy, Clone)]
pub enum MotorEncoderUnits {
    /// The units are in degrees
    #[default] Degrees,
    /// The units are in rotations
    Rotations,
    /// The units are in ticks
    Ticks,
}

/// The break mode of a motor
#[derive(Default, Copy, Clone)]
pub enum MotorBrakeMode {
    /// The motor will coast to a stop
    #[default] Coast,
    /// The motor will brake to a stop
    Brake,
    /// The motor will attempt to hold its current position
    /// reacting to outside forces
    Hold,
}

/// A basic smart motor
#[derive(Clone)]
pub struct SmartMotor {
    /// The smart port that this motor is connected to
    port: u32,
}

impl SmartMotor {
    /// Sets the motor's encoder units
    pub fn set_encoder_units(&mut self, units: MotorEncoderUnits) {
        unsafe {
            vexv5rt::vexDeviceMotorEncoderUnitsSet(self.get_vex_device(0), units as u32);
        }
    }

    /// Sets the motor's brake mode
    pub fn set_brake_mode(&mut self, mode: MotorBrakeMode) {
        unsafe {
            vexv5rt::vexDeviceMotorBrakeModeSet(self.get_vex_device(0), mode as u32);
        }
    }
}


impl Device for SmartMotor {
    fn init(&mut self) {
        // Set the encoder ticks to default
        self.set_encoder_units(MotorEncoderUnits::default());

        // Set the break mode to default
        self.set_brake_mode(MotorBrakeMode::default());

    }

    fn calibrate(&mut self) {
        // Reset the encoder
        self.reset_encoder();
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

impl Encoder for SmartMotor {
    fn get_ticks(&self) -> f64 {
        unsafe {
            vexv5rt::vexDeviceMotorPositionGet(self.get_vex_device(0))
        }
    }

    fn get_rate(&self) -> f64 {
        todo!()
    }

    fn reset_encoder(&mut self) {
        unsafe {
            vexv5rt::vexDeviceMotorPositionReset(self.get_vex_device(0));
        }
    }
}