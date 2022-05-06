use crate::hardware::devices::{SmartPort, Device, SmartDevice, Encoder};


/// Enum of what faults a motor is experiencing
#[derive(Copy, Clone)]
pub enum MotorFaults {
    /// No faults
    None,
    /// The motor is over temperature
    OverTemp,
    /// The h-bridge is faulting
    HBridgeFault,
    /// The motor is over current
    OverCurrent,
    /// The h-bridge is over current
    HBridgeOverCurrent,
}

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

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorEncoderUnitsSet(self.get_vex_device(0), units as u32);
        }
    }

    /// Sets the motor's brake mode
    pub fn set_brake_mode(&mut self, mode: MotorBrakeMode) {

        // Lock the device
        let _mtx = self.lock();


        unsafe {
            vexv5rt::vexDeviceMotorBrakeModeSet(self.get_vex_device(0), mode as u32);
        }
    }

    /// Sets the voltage of the motor, clampung it to the range -127 to 127
    pub fn move_voltage(&mut self, voltage: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Clamp the voltage to the range -127 to 127
        let voltage = voltage.min(127).max(-127);
        // Set the voltage
        unsafe {
            vexv5rt::vexDeviceMotorVoltageSet(self.get_vex_device(0), voltage);
        }
    }

    /// Moves the motor to a position at the given speed
    pub fn move_absolute(&mut self, position: f64, speed: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Move the motor
        unsafe {
            vexv5rt::vexDeviceMotorAbsoluteTargetSet(self.get_vex_device(0), position, speed);
        }
    }

    /// Moves the motor to a position relative to its current position
    /// at the given speed
    pub fn move_relative(&mut self, position: f64, speed: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Move the motor
        unsafe {
            vexv5rt::vexDeviceMotorRelativeTargetSet(self.get_vex_device(0), position, speed);
        }
    }

    /// Sets the velocity of the motor
    pub fn move_velocity(&mut self, velocity: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Set the velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocitySet(self.get_vex_device(0), velocity);
        }
    }

    /// Stops the motor
    pub fn stop(&mut self) {

        // Lock the device
        let _mtx = self.lock();

        // Stop the motor
        unsafe {
            vexv5rt::vexDeviceMotorVelocitySet(self.get_vex_device(0), 0);
        }
    }


    /// Updates the target velocity for the function move_relative and move_absolute
    pub fn set_target_velocity(&mut self, velocity: i32) {
            
        // Lock the device
        let _mtx = self.lock();

        // Set the target velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocityUpdate(self.get_vex_device(0), velocity)
        }
    }

    /// Gets the target velocity
    pub fn get_target_velocity(&mut self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the target velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocityGet(self.get_vex_device(0))
        }
    }

    /// Gets the target position
    pub fn get_target_position(&mut self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the target position
        unsafe {
            vexv5rt::vexDeviceMotorTargetGet(self.get_vex_device(0))
        }
    }
    

    /**************************************************************************
     * Telemetry functions                                                    *
     **************************************************************************/
    
    /// Get the velocity of the motor
    pub fn get_velocity(&mut self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocityGet(self.get_vex_device(0))
        }
    }

    /// Get how much current the motor is drawing in mA
    pub fn get_current(&mut self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the current
        unsafe {
            vexv5rt::vexDeviceMotorCurrentGet(self.get_vex_device(0))
        }
    }

    /// Get the direction the motor is spinning in
    // 1 for forward, -1 for reverse
    pub fn get_direction(&mut self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the direction
        unsafe {
            vexv5rt::vexDeviceMotorDirectionGet(self.get_vex_device(0))
        }
    }

    /// Gets the efficiency of the motor in percent.
    /// 100% is the motor is moving but drawing no power, 0% is the motor is drawing
    /// power but not moving.
    pub fn get_efficiency(&mut self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the efficiency
        unsafe {
            vexv5rt::vexDeviceMotorEfficiencyGet(self.get_vex_device(0))
        }
    }

    /// Returns a bitmask of the faults that have occured on the motor
    pub fn get_faults(&mut self) -> MotorFaults {

        // Lock the device
        let _mtx = self.lock();

        // Get the faults
        unsafe {
            vexv5rt::vexDeviceMotorFaultsGet(self.get_vex_device(0))
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

    fn get_smart_ports(&self) -> alloc::vec::Vec<(u32, SmartPort)> {
        vec![(self.port, SmartPort::Motor)]
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

    fn get_smart_port_type(&self) -> SmartPort {
        SmartPort::Motor
    }
}

impl Encoder for SmartMotor {
    fn get_ticks(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorPositionGet(self.get_vex_device(0))
        }
    }

    fn get_rate(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            <f64>::from(vexv5rt::vexDeviceMotorVelocityGet(self.get_vex_device(0))) * 6.0f64 // Converting from rpm to degrees/sec
        } 
    }

    fn reset_encoder(&mut self) {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorPositionReset(self.get_vex_device(0));
        }
    }
}