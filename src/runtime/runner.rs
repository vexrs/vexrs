use core::{arch::asm, ffi::c_void};

use alloc::vec::Vec;

use crate::println;

use super::{task::{Task, TaskState, TaskContext}, MAX_TASKS, DEFAULT_STACK_SIZE, guard};


// The global runtime
static mut RUNTIME: usize = 0;

/// A very simple round-robin scheduler
pub struct Runtime {
    pub tasks: Vec<Task>,
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
            tasks: tasks.clone(),
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
            //vexv5rt::vexSystemTimerReinitForRtos(1, Some(tick));
            
        }
    }

    /// Switches to the next context
    pub fn context_switch(&mut self) -> bool {

        // Find the next task to run
        let mut pos = self.current;
        loop {
            pos += 1;
            if pos >= self.tasks.len() {
                pos = 0;
            }
            if pos == self.current {
                return false;
            }
            if self.tasks[pos].state == TaskState::Ready {
                break;
            }
        }

        // Save the old index
        let old = self.current;
        
        // And set the current index
        self.current = pos;
        
        // Set the old task as ready
        self.tasks[old].state = TaskState::Ready;
        
        // And the new one as running
        self.tasks[self.current].state = TaskState::Running;
        
        // Get the stack pointer of the new context
        let sp = self.tasks[self.current].context.sp;
        
        println!("reached");
        crate::util::block(1000);



        unsafe {
            
            asm!(
                "ldr r0, =2",
                "push {{r0, {0}, r12, r11, r10, r9, r8, r7, r6, r5, r4, r3, r2, r1, r0}}",
                "mov {1}, sp",
                "mov r0, {2}",
                "b load_context",
                "2:",
                in(reg) guard as u32,
                out(reg) self.tasks[old].context.sp,
                in(reg) sp,
            );
        }
        
        true
    }

    /// Kills the current task
    pub fn kill_current(&mut self) {
        self.tasks[self.current].state = TaskState::Available;
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
                break;
            }
        }

        // Initialize it with the new entry

        // Clear it's stack
        self.tasks[pos].stack = vec![0u8; DEFAULT_STACK_SIZE];

        
        // Get the stack pointer
        let sp = unsafe { self.tasks[pos].stack.as_mut_ptr().offset(self.tasks[pos].stack.len() as isize)} as *mut u32;
        

        // Push pc as the entry point
        unsafe { core::ptr::write(sp.offset(-1), entry as u32);}

        // The guard function is here to prevent the task from returning to nothing.
        unsafe { core::ptr::write(sp.offset(-2), guard as u32) }
        println!("0x{:x}", sp as u32);
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