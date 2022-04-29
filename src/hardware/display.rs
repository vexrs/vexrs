// Basic library for interacting with the vex v5 display.


use alloc::{vec::Vec, boxed::Box};

use crate::{runtime::mutex::Mutex, println};

const BRAIN_SCREEN_WIDTH: u32 = 480;
const BRAIN_SCREEN_HEIGHT: u32 = 240;


/// Trait that defines objects that can be displayed
pub trait DisplayElement {
    /// Draws the shape, assuming the display is already locked
    fn draw(&self);

    /// Returns true if the given point intersects the shape
    fn intersects(&self) -> bool;

    /// Runs when a display element is tapped on
    fn pressed(&mut self) {}

    /// Runs when a display element is released
    fn released(&mut self) {}
}

/// A shape that can be drawn
pub enum Shape {
    Rectangle {x1: i32, y1: i32, x2: i32, y2: i32, color: u32, fill: bool},
    Circle {x: i32, y: i32, r: i32, color: u32, fill: bool}
}

impl DisplayElement for Shape {

    /// Draws the shape
    fn draw(&self) {
        match *self {
            Shape::Rectangle { x1, y1, x2, y2, color, fill } => {
                // Draw it using the v5 api
                if fill {
                    println!("{}", x1);
                    unsafe {
                        vexv5rt::vexDisplayForegroundColor(color);
                        vexv5rt::vexDisplayRectFill(x1, y1, x2, y2);
                    }
                } else {
                    unsafe {
                        vexv5rt::vexDisplayForegroundColor(color);
                        vexv5rt::vexDisplayRectDraw(x1, y1, x2, y2);
                    }
                }
            },
            Shape::Circle { x, y, r, color, fill } => {
                // Draw it using the v5 api
                if fill {
                    unsafe {
                        vexv5rt::vexDisplayForegroundColor(color);
                        vexv5rt::vexDisplayCircleFill(x, y, r);
                    }
                } else {
                    unsafe {
                        vexv5rt::vexDisplayForegroundColor(color);
                        vexv5rt::vexDisplayCircleDraw(x, y, r);
                    }
                }
            }
        };
    }

    fn intersects(&self) -> bool {
        false
    }
}

/// A Structure for interacting with the v5 brain display
pub struct Display {
    elements: Mutex<Vec<Box<dyn DisplayElement>>>,
    draw_lock: Mutex<()>
}

impl Display {

    /// Add a component to the display
    pub fn add(&mut self, element: Box<dyn DisplayElement>) {
        // Lock the mutex
        let mut list = self.elements.acquire();

        // Add the elements
        list.push(element);
    }

    /// Creates a new display object
    pub fn new() -> Display {
        Display {
            elements: Mutex::new(Vec::new()),
            draw_lock: Mutex::new(())
        }
    }

    /// Initializes the display, adding it to the global singleton
    pub fn init(&self) {
        unsafe {
            // Set the global runtime
            super::DISPLAY = self as *const Display;
        }
    }

    /// Clears the screen
    pub fn clear(&self) {
        
        // Lock the draw lock
        let mtx = self.draw_lock.acquire();

        // Lock the elements
        let mut elements = self.elements.acquire();

        // Clear the elements
        elements.clear();
        // Clear the screen
        unsafe {
            vexv5rt::vexDisplayErase();
        }
        
    }

    /// Draws a frame of the display
    pub fn draw(&self) {

        // Acquire a lock on the elements
        let elements = self.elements.acquire();

        // Acquire a lock on drawing
        let mtx = self.draw_lock.acquire();

        // Iterate over elements, drawing each
        for element in elements.iter() {
            // Draw the element
            element.draw();
        }
    }
    
}