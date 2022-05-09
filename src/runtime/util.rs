// Runtime utilities.
// This file is also exported by the main util.rs file

use core::arch::asm;

use crate::hardware::timer::Timer;

use super::runner::Runtime;

/// Gets the global runtime
pub fn get_runtime<'a>() -> &'a Runtime {
    &crate::RUNTIME
}

// A utility function to get the stack pointer
pub fn get_stack_pointer() -> u32 {
    unsafe {
        let sp;
        asm!(
            "mov {}, r13",
            out(reg) sp
        );
        sp
    }
}

/// Sleeps for t miliseconds.
pub fn sleep(t: u32) {
    // Yield for t miliseconds
    get_runtime().yield_for(t);
}

/// Blocks for t miliseconds
pub fn block(t: u32) {
    let timer = Timer::new(t);
    while !timer.is_elapsed() {}
}