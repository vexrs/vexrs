//! Vexrs only implements two contexts: the "supervisor" context and the "user" context.
//! For now there is a very primitive context switching system: every time that there is an IRQ, we switch.

use crate::libv5rt;

use core::ptr::addr_of;

/// Allocated one kilobyte of stack for both the user and system
pub const STACK_SIZE: usize = 0x1000;

/// The supervisor stack
static mut SUPERVISOR: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut SUPERVISOR_OFFSET: usize = 0;

/// The user stack
static mut USER: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut USER_OFFSET: usize = 0;

/// Enabled when the runtime is enabled
static mut RT_ENABLED: bool = false;

unsafe extern "C" fn timer_handler(data: *mut cty::c_void) {
    libv5rt::vexSystemTimerClearInterrupt();
}


/// A stack guard which makes it impossible to accidentally return to null
#[no_mangle]
pub unsafe extern "C" fn guard() {

    // Panic
    panic!("End of program.")
}

/// Initialized a given stack with the specified entry function
unsafe fn initialize_stack_with(stack: &mut [u8; STACK_SIZE], stack_offset: &mut usize, entry: fn()) {

    

    // Set the entire stack to zeroes
    *stack = [0u8; STACK_SIZE];

    // Get the top of the stack
    let top = (addr_of!(stack) as usize + STACK_SIZE) as *mut usize;

    // The stack structure upon context switch looks like this:
    // Saved PC
    // Saved LR
    // The rest of the saved registers.

    // Push PC as the entry point
    core::ptr::write(top.offset(-1), entry as usize);

    // Push the guard function to prevent us from returning to null
    core::ptr::write(top.offset(-2), guard as usize);

    // We are going to set the offset to 15 for all of the registers
    *stack_offset = 15;

}



fn supervisor_entry() {
    loop {}
}

/// Initialize the runtime
pub fn initialize() {


    
    unsafe {
        // Stop the system timer
        libv5rt::vexSystemTimerStop();

        // Initialize the supervisor stack
        initialize_stack_with(&mut SUPERVISOR, &mut SUPERVISOR_OFFSET, supervisor_entry);

        // Reconfigure the system timer
        libv5rt::vexSystemTimerReinitForRtos(30 << 3, Some(timer_handler));
    }

}