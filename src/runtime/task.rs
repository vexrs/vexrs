use alloc::vec::Vec;

use super::DEFAULT_STACK_SIZE;
use super::guard;





/// The state of a task determines when it should be entered again
/// and if it is still running.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TaskState {
    /// In the ready state, a task is waiting for it's turn on the scheduler
    Ready,
    /// A running task is the currently executing task
    Running,
    /// An available task has nothing assigned to it.
    Available,
    /// A task that is waiting until a specific time
    WaitUntil(u32),
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
            id,
            stack: vec![0u8; DEFAULT_STACK_SIZE],
            context: TaskContext::default(),
            state: TaskState::Available
        }
    }

    /// Creates a new task in it's ready state
    pub fn new_ready(id: usize, entry: fn())  -> Task {
        
        // Create a new stack for this task
        let mut stack: Vec<u8> = vec![0; DEFAULT_STACK_SIZE];
        let sp = unsafe { stack.as_mut_ptr().add(stack.len())} as *mut usize;

        

        // Push pc as the entry point
        unsafe { core::ptr::write(sp.offset(-1), entry as usize);}

        // The guard function is here to prevent the task from returning to nothing.
        unsafe { core::ptr::write(sp.offset(-2), guard as usize) }
        
        
        // Create a context
        let context = TaskContext {
            sp: unsafe { sp.offset(-15) } as u32,
        };
        
        Task {
            id,
            stack,
            context,
            state: TaskState::Ready,
        }
    }
}



