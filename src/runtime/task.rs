use core::arch::asm;
use crate::println;
use alloc::vec::Vec;

use super::DEFAULT_STACK_SIZE;
use super::guard;





/// The state of a task determines when it should be entered again
/// and if it is still running.
#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Clone, Copy, Default, Debug)]
pub struct TaskContext {
    pub sp: u32,
}


/// A task is a single unit which contains its own stack, a context
/// containing saved registers, and a state which determines whether
/// the task is ready to run or not
#[derive(Clone, Debug)]
pub struct Task {
    /// This task's ID
    pub id: usize,
    /// The stack for this task
    pub stack: Vec<u8>,
    /// The context for this task
    pub context: TaskContext,
    /// The state of this task
    pub state: TaskState,
}

impl Task {

    /// Creates a new, empty task
    pub fn new(id: usize) -> Task {
        Task {
            id: id,
            stack: vec![0u8; DEFAULT_STACK_SIZE],
            context: TaskContext::default(),
            state: TaskState::Available
        }
    }

    /// Creates a new task in it's ready state
    pub fn new_ready(id: usize, entry: fn())  -> Task {
        
        // Create a new stack for this task
        let mut stack: Vec<u8> = vec![0; DEFAULT_STACK_SIZE];
        let sp = unsafe { stack.as_mut_ptr().offset(stack.len() as isize)} as *mut u32;

        

        // Push pc as the entry point
        unsafe { core::ptr::write(sp.offset(-1), entry as u32);}

        // The guard function is here to prevent the task from returning to nothing.
        unsafe { core::ptr::write(sp.offset(-2), guard as u32) }
        
        
        // Create a context
        let context = TaskContext {
            sp: unsafe { sp.offset(-15) } as u32,
        };
        
        Task {
            id,
            stack: stack,
            context,
            state: TaskState::Ready,
        }
    }
}



/// Unsafe function that switches to a different task's context.
/// Internal use only. This does not save the current context, only loads a new one.
/// The stack pointer of the new context should be passed as r0
#[naked]
#[no_mangle]
pub unsafe extern "C" fn load_context() {
    // Set the stack pointer
    // Pop all required registers
    asm!(
        "mov sp, r0",
        "pop {{r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12}}",
        "pop {{lr, pc}}",
        options(noreturn),
    );
    
}