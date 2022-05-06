// A structure that manages the state of all devices connected to the brain

use alloc::vec::Vec;

use crate::runtime::mutex::Mutex;

use super::SmartPort;




/// Manages the persistent state of all devices connected to the brain
struct DeviceManager {
    /// All the smart ports connected to the V5 brain.
    /// Note that each smart port does not mean a single device.
    /// For example, ADI expanders can have 8 ADI devices, and
    /// motors have both encoders and motors.
    smart_ports: [Mutex<SmartPort>; 24],
    /// This is a vector of all devices on the robot brain.
    /// Devices can only be added to this vector if it is confirmed that their
    /// smart port is not occupied. Once it is comfirmed, their port is reserved.
    devices: Vec<Mutex<dyn Device>>,
}