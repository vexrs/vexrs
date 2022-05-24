// Contains the implementation of a thread

use core::intrinsics::size_of;

use alloc::vec::{Vec};
use alloc::vec;

/// The size of a thread's stack
pub const STACK_SIZE: usize = 0x1000; // 4 KiB for now should be plenty.

/// The state of a thread
#[derive(Clone, PartialEq)]
pub enum ThreadState {
    Available,
    Ready,
    Running,
}



/// A thread that contains the utilities for switching between contexts
/// The thread struct should *never* be put into any relocatable data structure such as a 
/// Vec.
#[derive(Clone)]
pub struct Thread {
    /// The contents of the stack
    stack: Vec<u8>,
    /// The current offset in the stack
    stack_offset: usize,
    /// The current thread state
    pub state: ThreadState,
}

impl Thread {

    /// Creates a new empty thread
    pub fn new() -> Thread {
        Thread { stack: vec![0u8; STACK_SIZE], stack_offset: 0, state: ThreadState::Available }
    }

    /// Initializes the thread to be ready
    pub fn initialize(&mut self, entry: fn()) {
        // Initialize a new stack
        self.stack = vec![0u8; STACK_SIZE];

        

        // Get the stack top
        let top = (core::ptr::addr_of!(self.stack) as usize + self.stack.len()) as *mut usize;

        unsafe {
            // Push pc as the entry point
            core::ptr::write(top.offset(-1), entry as usize);

            // Push the guard function to prevent us from returning to null
            core::ptr::write(top.offset(-2), super::internal::guard as usize);
        }
        
        // Set our default offset to 15 usizes from the top (14 registers, one indexed)
        self.stack_offset = 15;

        // Set our state to ready
        self.state = ThreadState::Ready;
    }

    /// Gets the stack pointer of this thread
    pub fn get_sp(&self) -> usize {
        // Add the size of the stack to the address of the stack to get the end of the stack,
        // and subtract the offset to get the stack pointer of this thread. We do this to assist
        // in the case of this struct getting relocated while the thread is suspended so that we do not have to reset the stack pointer each time.
        core::ptr::addr_of!(self.stack) as usize + self.stack.len() - self.stack_offset * size_of::<usize>()
    }

    /// Switches contexts from a different thread to the stack pointer of a different thread
    /// # Safety
    /// This function assumes the stack pointer is correct.
    pub unsafe fn switch_from(&self, to: usize) {
        // This function will return to the new context.
        // The way this works follows:
        // 1. A program calls this function
        // 2. The current thread state is saved
        // 3. This function loads the new thread's state
        // 4. This function returns to the new thread.
        // To a thread it appears as if this function simply returns and execution continues as normal. However, during the execution of this function
        // that thread is suspended.

        // Get the address of the stack offset variable
        let so_addr = core::ptr::addr_of!(self.stack_offset);

        // Get the end of the current stack
        let stack_end = core::ptr::addr_of!(self.stack) as usize + self.stack.len();
        
        
        core::arch::asm!(
            "ldr {1}, =2f", // Load the label 2 into the scratch register (this is where we want to jump to when our thread resumes execution)
            "push {{{1}}}", // Push the end label as the saved program counter
            "push {{{0}}}", // Push the guard function as the link register (this should be overwritten when a function returns)
            "push {{r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12}}", // Push the general purpose registers
            "sub {4}, sp", // Convert the current stack pointer to an offset 
            "lsr {4}, 2", // Divide the offset by four in order to get the offset in usizes.
            "str {4}, [{2}]", // Save the stack offset
            "mov sp, {3}", // Load the new stack pointer (we are now on a new stack! yay!
            "pop {{r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12}}", // Pop the general purpose registers
            "pop {{lr}}", // Pop the link register
            "pop {{pc}}", // Pop the program counter, finishing up the context switch
            "2:",
            in(reg) super::internal::guard as usize, // Store the stack guard inj a register
            out(reg) _, // A scratch register to use
            in(reg) so_addr, // Store the address of the stack pointer variable in a register
            in(reg) to, // The stack pointer of the new thread
            in(reg) stack_end, // The current stack end address
        );
        
    }
}

impl Default for Thread {
    fn default() -> Thread {
        Thread::new()
    }
}