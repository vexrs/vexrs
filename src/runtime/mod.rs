

/// Private utility functions
mod internal;

/// A thread implementation
pub mod thread;

/// We max out the number of threads at 8. This can be changed to support
// custom needs
const MAX_THREADS: usize = 8;

/// The runtime implementation that maintains a list of threads
/// as well as information about the current thread and a round robin scheduler.
struct Runtime {
    /// The list of currently running thread
    /// Three threads are treated specially.
    /// Thread zero is the kernel thread and when exited will kill all other threads
    /// and restarts itself.
    /// Thread one is the user thread and is restarted every time the competition mode changes.
    /// Thread two is the user tick thread and is never killed except when the kernel thread is killed/
    /// All other threads are ignored by the kernel.
    threads: [thread::Thread; MAX_THREADS],
    /// The index of the current thread
    current_thread: usize,
}