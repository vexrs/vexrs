
// Various core and alloc imports that are used by the runtime
use core::{cell::RefCell, sync::atomic::{AtomicUsize, Ordering}};

use self::thread::ThreadState;

/// Private utility functions
mod internal;

/// A thread implementation
pub mod thread;

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
    threads: RefCell<[thread::Thread; MAX_THREADS]>,
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
            threads: RefCell::new(threads),
            current: AtomicUsize::new(0),
        }
    }

    /// Switches to a thread with a given index
    fn context_switch(&self, next: usize) {
        // Save the current thread
        let current = self.current.load(Ordering::SeqCst);

        // Update the current thread
        self.current.store(next, Ordering::SeqCst);

        // Borrow threads as mutable
        let mut threads = self.threads.borrow_mut();

        // Set the current as running and the next one as ready
        threads[current].state = ThreadState::Ready;
        threads[next].state = ThreadState::Running;

        // Context switch
        unsafe {

            // Context switch to the next thread. We borrow twice to allow this to happen.
            // TODO: This is probably unsafe and should not happen. However, green-threading is mostly unsafe
            // so this may be a compromise that needs to be made.
            self.threads.borrow_mut()[current].switch_from(&self.threads.borrow()[next]);
        }
    }

    /// Gets the next thread to run, returns None if it iterates back to the current thread.
    fn get_next(&self) -> Option<usize> {
        for (i,thread) in self.threads.borrow().iter().enumerate() {
            match thread.state {
                ThreadState::Ready => {
                    return Some(i);
                },
                ThreadState::Running => {
                    return None;
                },
                _ => {}
            };
        }

        None
    }

    /// Yields to the next thread
    pub fn yield_next(&self) {
        // Get the next thread to run
        let next = self.get_next();

        // If there is a thread to switch to, then switch
        if let Some(n) = next {
            self.context_switch(n);
        }
    }
}