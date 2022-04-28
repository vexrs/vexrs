// A timer implementation

/// A basic timer implementation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timer {
    // When the timer ends
    ends: u32,
    // When the timer started
    starts: u32,
}

impl Timer {
    /// Creates a new timer that will last t miliseconds
    pub fn new(t: u32) -> Timer {
        let starts = Timer::current_time();
        Timer {
            starts,
            ends: starts + t,
        }
    }

    /// Creates a new timer that will last until t miliseconds
    pub fn new_until(t: u32) -> Timer {
        let starts = Timer::current_time();
        Timer {
            starts,
            ends: t,
        }
    }

    /// Blocks until the timer has elapsed
    pub fn block(&self) {
        while !self.is_elapsed() {}
    }

    /// Returns true if the timer has elapsed
    pub fn is_elapsed(&self) -> bool {
        Timer::current_time() >= self.ends
    }

    /// Returns the current system time
    pub fn current_time() -> u32 {
        unsafe {
            vexv5rt::vexSystemTimeGet()
        }
    }

    /// Gets the time at which the timer ends
    pub fn get_end_time(&self) -> u32 {
        self.ends
    }

    /// Gets the time at which the timer started
    pub fn get_start_time(&self) -> u32 {
        self.starts
    }

    /// Returns how much longer until the current timer ends
    pub fn get_remaining(&self) -> u32 {
        if self.is_elapsed() {
            0
        } else {
            self.ends - Timer::current_time()
        }
    }

    /// Returns how much time has elapsed
    pub fn get_elapsed(&self) -> u32 {
        if self.is_elapsed() {
            self.ends - self.starts
        } else {
            Timer::current_time() - self.starts
        }
    }
}