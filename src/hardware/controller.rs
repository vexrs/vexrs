// Interfaces for the vex v5 controller


use alloc::string::{String, ToString};

use crate::runtime::mutex::Mutex;

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

/// Controller Status Register indexes
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum ControllerRegister {
    BatteryLevel = 19,
    BatteryCapacity = 22,
}


/// Struct that allows user programs to interact with the vex v5 controller
pub struct Controller {
    id: Mutex<ControllerID>,
    button_status: Mutex<[bool; 12]>
}

impl Controller {
    /// Creates a new controller with id.
    pub fn new(id: ControllerID) -> Controller {
        Controller {
            id: Mutex::new(id),
            button_status: Mutex::new([false; 12]),
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

        // Lock the button status
        let _button_status = self.button_status.acquire();

        // Get the index of the button
        let index = button as u32;

        // Get the button value
        let button_value = unsafe {
            vexv5rt::vexControllerGet(*id as u32, index)
        };

        // Return the button status as true if it is not zero.
        button_value != 0
    }

    /// Returns true if a button has just been pressed
    pub fn get_button_pressed(&self, button: ControllerButton) -> bool {
        // Get the value of the button in the cache
        let btn_status=  self.button_status.acquire()[button as usize - 6];

        // Get the new status
        let new_value = self.get_digital(button);

        // If the values are different and the new value is true, the button has been newly pressed
        if btn_status != new_value {
            // Set it in the cache
            self.button_status.acquire()[button as usize-6] = new_value;
            new_value
        } else {
            false
        }
    }

    /// Returns true if a button has just been released
    pub fn get_button_released(&self, button: ControllerButton) -> bool {
        // Get the value of the button in the cache
        let btn_status=  self.button_status.acquire()[button as usize - 6];

        // Update the button status
        let new_value = self.get_digital(button);

        // If the values are different and the new value is false, the button has been newly released
        if btn_status != new_value {
            // Set it in the cache
            self.button_status.acquire()[button as usize-6] = new_value;
            !new_value
        } else {
            false
        }
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

    /// Set text on the controller screen
    pub fn set_text(&self, x: u32, y: u32, text: String) {

        // Lock the controller ID
        let id = self.id.acquire();

        // Redefine text as mutable
        let mut text = text;

        // Add a \0 to the text
        text.push('\0');

        // Set the text
        unsafe {
            vexv5rt::vexControllerTextSet(*id as u32, y+1, x, text.as_ptr());
        }
    }

    /// Clear the controller's screen
    pub fn clear_screen(&self) {

        // Just set the text at 255, which should work in the latest VexOS
        // version according to the PROS source code
        self.set_text(0, 255, "".to_string())
    }

    /// Rumbles the controller using a passed pattern
    /// Dots (.) are short rumbles, dashes (-) are long rumbles
    /// and spaces ( ) are, well, spaces.
    pub fn rumble(&self, pattern: String) {
        // According to PROS, if you set line # 3 to the 
        // rumble pattern, the controller will rumble
        self.set_text(0, 3, pattern);
    }

    /// Gets the controller's battery capacity
    pub fn battery_capacity(&self) -> i32 {
        // Lock the controller's id
        let id = self.id.acquire();

        unsafe {
            vexv5rt::vexControllerGet(*id as u32, ControllerRegister::BatteryCapacity as u32)
        }
    }

    /// Gets the controller's battery level
    pub fn battery_level(&self) -> i32 {
        // Lock the controller's id
        let id = self.id.acquire();

        unsafe {
            vexv5rt::vexControllerGet(*id as u32, ControllerRegister::BatteryLevel as u32)
        }
    }
}