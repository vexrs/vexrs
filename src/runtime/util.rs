// Runtime utilities.
// This file is also exported by the main util.rs file

use core::arch::asm;

use super::runner::Runtime;

/// Gets the global runtime
pub fn get_runtime() -> &'static mut Runtime {
    unsafe {
        let rt = super::RUNTIME as *mut Runtime;
        &mut *rt
    }
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
