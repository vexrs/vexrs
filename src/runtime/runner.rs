use core::{arch::asm, ffi::c_void};

use alloc::vec::Vec;

use crate::println;

use super::{task::{Task, TaskState, TaskContext}, MAX_TASKS, DEFAULT_STACK_SIZE, guard};


// The global runtime
static mut RUNTIME: usize = 0;

/// A very simple round-robin scheduler
pub struct Runtime {
    tasks: Vec<Task>,
    current: usize
}


impl Runtime {

    /// Creates a new runtime
    pub fn new() -> Runtime {
        
        let base_task = Task {
            id: 0,
            stack: vec![0u8; DEFAULT_STACK_SIZE],
            context: TaskContext::default(),
            state: TaskState::Running,
        };
        
        // Create a vector of tasks
        let mut tasks = vec![base_task];
        // And fill it with MAX_TASKS new tasks
        for i in 1..MAX_TASKS {
            tasks.push(Task::new(i));
        }
        

        Runtime {
            tasks: tasks,
            current: 0
        }
    }




    /// This function sets our runtime to the global runtime variable
    /// and registers the timer interupt
    pub fn init(&self) {
        unsafe {
            // Set the global runtime
            RUNTIME = self as *const Runtime as usize;
            
            // Register the timer interrupt
            vexv5rt::vexSystemTimerReinitForRtos(1, Some(tick));
            
        }
    }

    /// Switches to the next context
    pub fn context_switch(&mut self) {

        println!("Begin CTX switch: {:?}", self.tasks.clone());

        
        
    }



    /// Spawns a new task
    pub fn spawn(&mut self, entry: fn()) -> Task {
        // Find the next available task
        let mut pos = self.current;
        loop {
            pos += 1;
            if pos >= self.tasks.len() {
                pos = 0;
            }
            if self.tasks[pos].state == TaskState::Available {
                self.current = pos;
                break;
            }
        }

        // Initialize it with the new entry

        // Clear it's stack
        self.tasks[pos].stack.clear();

        
        // Get the stack pointer
        let sp = unsafe { self.tasks[pos].stack.as_mut_ptr().offset(self.tasks[pos].stack.len() as isize)} as *mut u32;
        

        // Push pc as the entry point
        unsafe { core::ptr::write(sp.offset(-1), entry as u32);}

        // The guard function is here to prevent the task from returning to nothing.
        unsafe { core::ptr::write(sp.offset(-2), guard as u32) }

        // Set the stack pointer
        self.tasks[pos].context = TaskContext {
            sp: unsafe { sp.offset(-15) } as u32,
        };

        // Set the task state to ready
        self.tasks[pos].state = TaskState::Ready;

        self.tasks[pos].clone()
    }
}



/// Gets the global runtime
pub fn get_runtime() -> &'static mut Runtime {
    unsafe {
        let rt = RUNTIME as *mut Runtime;
        &mut *rt
    }
}


/// This is the main tick function, called on every timer tick
#[no_mangle]
pub unsafe extern "C" fn tick(data: *mut c_void) {

    

    // Get the runtime
    let rt = get_runtime();
    

    // Switch to the next context
    rt.context_switch();

    // Clear the timer interrupt
    vexv5rt::vexSystemTimerClearInterrupt();

}