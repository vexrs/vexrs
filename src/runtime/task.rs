use core::arch::asm;

use alloc::{boxed::Box, vec::Vec};

use super::DEFAULT_STACK_SIZE;





/// The state of a task determines when it should be entered again
/// and if it is still running.
#[derive(Debug, PartialEq, Eq)]
pub enum TaskState {
    /// In the ready state, a task is waiting for it's turn on the scheduler
    Ready,
    /// A running task is the currently executing task
    Running,
    /// An available task has nothing assigned to it.
    Available
}

/// The context of a task contains all registers that need to be restored
/// when the task is resumed.
pub struct TaskContext {
    sp: u32,
    lr: u32,
}


/// A task is a single unit which contains its own stack, a context
/// containing saved registers, and a state which determines whether
/// the task is ready to run or not
pub struct Task {
    /// The stack for this task
    stack: Vec<u8>,
    /// The context for this task
    pub context: TaskContext,
    /// The state of this task
    state: TaskState,
}

impl Task {
    /// Creates a new task in it's ready state
    pub fn new(func: fn())  -> Task {

        // Create a new stack for this task
        let stack = vec![0; DEFAULT_STACK_SIZE];
        
        // Create a context
        let context = TaskContext {
            // We do 52 less in order to make room for the general purpose registers during a context switch.
            sp: (stack.as_ptr() as u32) + stack.len() as u32 - 52,
            lr: func as u32,
        };

        Task {
            stack: stack,
            context: TaskContext {
                sp: 0,
                lr: 0,
            },
            state: TaskState::Ready,
        }
    }
}



/// Unsafe function that switches to a different task's context.
/// Internal use only. This does not save the current context, only loads a new one.
pub unsafe fn load_context(ctx: TaskContext) {

    // Set the stack pointer
    // to the new stack
    asm!(
        "mov sp, {0}",
        in(reg) ctx.sp
    );

    
    crate::util::block(100);

    // Pop all of the required registers
    // from the stack
    asm!(
        "pop {{r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12}}",
    );
    

    // Set the link register
    asm!(
        "mov lr, {0}",
        in(reg) ctx.lr
    );

    // Return to the new context
}