// Contains the implementation of a thread

/// The size of a thread's stack
pub const STACK_SIZE: usize = 0x1000; // 4 KiB for now should be plenty.

/// A thread that contains the utilities for switching between contexts
pub struct Thread {
    /// The contents of the stack
    stack: [u8; STACK_SIZE],
    /// The current stack pointer
    sp: usize,
}

impl Thread {
    /// Switches contexts from a different thread to this thread
    /// # Safety
    /// This function does not run any checks on the state of the thread and assumes the thread's stack is setup properly.
    pub unsafe fn switch_from(&mut self, other: &Thread) {
        // This function will return to the new context.
        // The way this works follows:
        // 1. A program calls this function
        // 2. The current thread state is saved
        // 3. This function loads the new thread's state
        // 4. This function returns to the new thread.
        // To a thread it appears as if this function simply returns and execution continues as normal. However, during the execution of this function\
        // that thread was suspended.

        unsafe {
            core::arch::asm!(
                "ldr {1}, =2f", // Load the label 2 into the scratch register (this is where we want to jump to when our thread resumes execution)
                "push {{{1}}}", // Push the end label as the saved program counter
                "push {{{0}}}", // Push the guard function as the link register (this should be overwritten when a function returns)
                "push {{r12, r11, r10, r9, r8, r7, r6, r5, r4, r3, r2, r1, r0}}", // Push the general purpose registers
                "str sp, [{2}]", // Save the current stack pointer
                "mov sp, {3}", // Load the new stack pointer (we are now on a new stack! yay!
                "pop {{r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, lr}}", // Pop the general purpose registers and the link register
                "pop {{pc}}", // Pop the program counter, finishing up the context switch
                "2:"
                in(reg) guard as usize, // Store the stack guard inj a register
                out(reg) _, // A scratch register to use
                in(reg) core::ptr::addr_of!(self.sp), // Store the address of the stack pointer variable in a register
                in(reg) other.sp, // The stack pointer of the new task
            );
        }
    }
}