
// Various core and alloc imports that are used by the runtime
use core::{cell::UnsafeCell, sync::atomic::{AtomicUsize, Ordering}};

use self::thread::{ThreadState, Thread};

/// Private utility functions
mod internal;

/// A thread implementation
pub mod thread;

lazy_static::lazy_static! {
    /// The global runtime singleton
    pub static ref RUNTIME: Runtime = Runtime::new();
}


/// We max out the number of threads at 8. This can be changed to support
// custom needs
const MAX_THREADS: usize = 8;

/// The runtime implementation that maintains a list of threads
/// as well as information about the current thread and a round robin scheduler.
/// The runtime struct uses interior mutability to allow it to be used as a static singleton.
pub struct Runtime {
    /// The list of currently running thread
    /// Three threads are treated specially.
    /// Thread zero is the kernel thread and when exited will kill all other threads
    /// and restarts itself.
    /// Thread one is the user thread and is restarted every time the competition mode changes.
    /// Thread two is the user tick thread and is never killed except when the kernel thread is killed/
    /// All other threads are ignored by the kernel.
    threads: UnsafeCell<[thread::Thread; MAX_THREADS]>,
    /// The index of the current thread
    current: core::sync::atomic::AtomicUsize,
}


impl Runtime {
    /// Creates and initializes a new runtime
    pub fn new() -> Runtime {
        // Create the OS thread
        let mut os = thread::Thread::new();

        // Set it as running
        os.state = thread::ThreadState::Running;

        // Create the thread list
        let mut threads: [thread::Thread; MAX_THREADS] = Default::default();

        // Set the OS thread
        threads[0] = os;

        // Return the runtime
        Runtime {
            threads: UnsafeCell::new(threads),
            current: AtomicUsize::new(0),
        }
    }

    /// Switches to a thread with a given index
    unsafe fn context_switch(&self, next: usize) {
        // Save the current thread
        let current = self.current.load(Ordering::SeqCst);

        // Update the current thread
        self.current.store(next, Ordering::SeqCst);

        // Get threads as mutable
        let mut threads = self.threads.get();

        // Set the current as running and the next one as ready
        (*threads)[current].state = ThreadState::Ready;
        (*threads)[next].state = ThreadState::Running;

        // Context switch to the next thread.
        // Get the next thread's stack pointer
        let t = (*threads)[next].get_sp();
        
        // Run the actual context switch
        (*threads)[current].switch_from(t);
        
    }

    /// Gets the next thread to run, returns None if it iterates back to the current thread.
    fn get_next(&self) -> Option<usize> {
        let mut i = self.current.load(Ordering::SeqCst);
        let threads = self.threads.get();
        loop {
            i+=1;
            if i >= unsafe {(*threads).len()} {
                i = 0;
            }
            unsafe {
                match (*threads)[i].state {
                    ThreadState::Ready => {
                        return Some(i);
                    },
                    _ => {}
                };
            }
            if i == self.current.load(Ordering::SeqCst) {
                return None;
            }
        }
    }

    /// Yields to the next thread
    pub fn yield_next(&self) {
        // Get the next thread to run
        let next = self.get_next();

        

        // If there is a thread to switch to, then switch
        if let Some(n) = next {
            unsafe { self.context_switch(n); }
        }
    }


    /// Spawns a new thread
    pub fn spawn(&self, entry: fn()) {

        // Find the next available thread
        let mut pos = self.current.load(Ordering::SeqCst);
        let threads = self.threads.get();

        loop {
            pos += 1;
            if pos >= unsafe { (*threads).len() } {
                pos = 0;
            }

            // If it is the same as the current, then to threads were found and we can not spawn the thread
            if pos == self.current.load(Ordering::SeqCst) {
                return;
            }

            if unsafe { (*threads)[pos].state == ThreadState::Available } {
                break;
            }
        }

        // Re-initialize the thread
        unsafe {
            (*threads)[pos].initialize(entry);
        }

        
    } 
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}


/// Force sync. This is bad practice but required for the runtime.
unsafe impl Sync for Runtime {}