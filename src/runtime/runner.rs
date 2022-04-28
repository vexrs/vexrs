// A basic green threads runtime

use core::arch::asm;
use alloc::vec::Vec;

use crate::{println, hardware::timer::Timer};
use super::{task::{Task, TaskState, TaskContext}, MAX_TASKS, DEFAULT_STACK_SIZE, guard};



/// A very simple round-robin scheduler
#[derive(Default)]
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
            super::RUNTIME = self as *const Runtime;
            
        }
    }


    /// Yields the current task, setting it as ready to be returned to on any notice.
    pub fn yield_t(&mut self) {
        self.yield_as(TaskState::Ready);
    }

    /// Yields the current task, and will not resume it until a time has been reached
    pub fn yield_until(&mut self, t: u32) {
        self.yield_as(TaskState::WaitUntil(Timer::new_until(t)));
    }

    /// Yields the current task, and will not resume until the specified delay has elapsed.
    pub fn yield_for(&mut self, t: u32) {
        self.yield_as(TaskState::WaitUntil(Timer::new(t)));
    }

    /// Switches to the next context, saving this task's state specialy.
    fn yield_as(&mut self, state: TaskState) -> bool {

        // Set this task state
        self.tasks[self.current].state = state;

        // Find the next task to run
        // If we find a ready task, 
        let mut pos = self.current;
        let mut has_waiting_task = false;
        loop {
            pos += 1;
            if pos >= self.tasks.len() {
                pos = 0;
            }
            
            
            if self.task_waiting(self.tasks[pos].id) {
                has_waiting_task = true;
            }
            if self.task_ready(self.tasks[pos].id) {
                break;
            }
            if pos == self.current && !has_waiting_task {
                return false;
            }
        }

        // Save the old index
        let old = self.current;
        
        // And set the current index
        self.current = pos;
        
        
        // And the new one as running
        self.tasks[self.current].state = TaskState::Running;
        



        // Run the actual context switch
        self.context_switch(self.current, old);

        true
    }

    /// Context switches to a new task
    fn context_switch(&mut self, new: usize, old: usize) {
        unsafe {
            asm!("/* {0} */",
                "ldr {1}, =2f",
                "push {{{1}}}", // For some reason we have to push these two separately
                "push {{{0}}}",
                "push {{r12, r11, r10, r9, r8, r7, r6, r5, r4, r3, r2, r1, r0}}",
                "str sp, [{2}]",
                "mov sp, {3}",
                "pop {{r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, lr, pc}}",
                "2:",
                in(reg) guard as usize,
                out(reg) _, // We just want to reserve a register to use
                in(reg) core::ptr::addr_of!(self.tasks[old].context.sp),
                in(reg) self.tasks[new].context.sp,
            );
        }
    }

    /// Returns if a task is ready to be executed
    pub fn task_ready(&mut self, id: usize) -> bool {
        // Find the task's index by it's id
        let pos = self.tasks.iter().position(|t| t.id == id);

        // If it was not found, return false
        let pos = match pos {
            Some(p) => p,
            _ => {
                return false;
            }
        };

        match self.tasks[pos].state {
            TaskState::Ready => true,
            TaskState::WaitUntil(t) => t.is_elapsed(),
            TaskState::Running => false,
            TaskState::Available => false,
        }
    }


    /// Returns true if a task is waiting to be executed
    pub fn task_waiting(&mut self, id: usize) -> bool {
        // Find the task's index by it's id
        let pos = self.tasks.iter().position(|t| t.id == id);

        // If it was not found, return false
        let pos = match pos {
            Some(p) => p,
            _ => {
                return false;
            }
        };

        match self.tasks[pos].state {
            TaskState::Ready => false,
            TaskState::WaitUntil(t) => !t.is_elapsed(),
            TaskState::Running => false,
            TaskState::Available => false,
        }
    }

    /// Yields to a specific task. Returns true if successful.
    pub fn wake(&mut self, id: usize) -> bool {
        // Find the task's index by it's id
        let pos = self.tasks.iter().position(|t| t.id == id);

        // If it was not found, return false
        let pos = match pos {
            Some(p) => p,
            _ => {
                return false;
            }
        };

        // If the new task is ready, then context switch
        if !self.task_ready(id) {
            return false;
        }

        // Save the current task
        let old = self.current;

        // And update the current task
        self.current = pos;

        // Update the old task's state to ready
        self.tasks[old].state = TaskState::Ready;

        // And the new one to running
        self.tasks[self.current].state = TaskState::Running;

        // Now context switch
        self.context_switch(self.current, old);

        true
    }
    
    /// Kills the current task
    pub fn kill_current(&mut self) {
        self.tasks[self.current].state = TaskState::Available;
    }

    /// Kills a task with the specified ID
    pub fn kill_task(&mut self, id: usize) {
        self.tasks[id].state = TaskState::Available;
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
        let sp = unsafe { self.tasks[pos].stack.as_mut_ptr().add(self.tasks[pos].stack.len())} as *mut usize;
        

        // Push pc as the entry point
        unsafe { core::ptr::write(sp.offset(-1), entry as usize);}

        // The guard function is here to prevent the task from returning to nothing.
        unsafe { core::ptr::write(sp.offset(-2), guard as usize) }
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



