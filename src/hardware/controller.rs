// Interfaces for the vex v5 controller

use crate::{runtime::mutex::Mutex, println};

/// The ID of a controller
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerID {
    Master,
    Partner
}

/// The status of a controller
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerStatus {
    Disconnected,
    Tethered,
    Wireless,
    Unknown,
}


/// A digital button on the controller
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerButton {
    L1 = 0x6,
    L2,
    R1, R2,
    Up, Down, Left, Right,
    X, B, Y, A,
}

/// An analog input on the controller
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerAxis {
    LeftX = 0,
    LeftY,
    RightX,
    RightY,
}

/// Struct that allows user programs to interact with the vex v5 controller
pub struct Controller {
    id: Mutex<ControllerID>,
}

impl Controller {
    /// Creates a new controller with id.
    pub fn new(id: ControllerID) -> Controller {
        Controller {
            id: Mutex::new(id)
        }
    }

    /// Returns an enum containing the controller's connection status
    pub fn connection_status(&self) -> ControllerStatus {

        // Lock the self mutex
        let id = self.id.acquire();

        // Get the controller status
        let status = unsafe {
            vexv5rt::vexControllerConnectionStatusGet(*id as u32)
        };

        match status {
            0 => ControllerStatus::Disconnected,
            1 => ControllerStatus::Tethered,
            2 => ControllerStatus::Wireless,
            _ => ControllerStatus::Unknown
        }
    }

    /// Gets the digital value of a button on the controller
    pub fn get_digital(&self, button: ControllerButton) -> bool {

        // Lock the self mutex
        let id = self.id.acquire();

        // Get the index of the button
        let index = button as u32;

        // Get the button status
        let button_status = unsafe {
            vexv5rt::vexControllerGet(*id as u32, index)
        };

        // Return the button status as true if it is not zero.
        button_status != 0
    }

    /// Gets the analog value of a button on the controller
    /// Analog values are -127 to 127
    pub fn get_analog(&self, axis: ControllerAxis) -> i32 {

        // Lock the self mutex
        let id = self.id.acquire();

        // Get the index of the axis
        let index = axis as u32;

        // Get the value of the axis
        unsafe {
            vexv5rt::vexControllerGet(*id as u32, index)
        }
    }
}