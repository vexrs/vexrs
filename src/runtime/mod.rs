// The core CEROS runtime support library.


use crate::{eprintln, system::os_init};


// A basic task structure implementation
pub mod task;

// The actual runtime, including a simple round-robin scheduler.
pub mod runner;

// A basic mutex implementation
pub mod mutex;

// Utility files for interacting with the runtime
pub mod util;

// Export most structs
pub use runner::*;
pub use task::*;
pub use util::*;
// A default stack size of 8192 bytes should be good.
// This will almost certainly made larger later, but we are using this for ease
// of development, and because this is what 
pub const DEFAULT_STACK_SIZE: usize = 0x2000;

// 8 Seems like a good number of tasks. We can increase this if we want.
pub const MAX_TASKS: usize = 8;





/// The entry point to the CEROS runtime.
pub fn main(user_entry: fn()) {
    // Initialize the os and begin ticking
    os_init(user_entry);
}

/// This function is the guard for the scheduler. It is called when
/// a function returns and there is nothing above the callstack.
#[no_mangle]
unsafe extern "C"  fn guard() {
    let rt = crate::util::get_runtime();

    // Kill the current task
    rt.kill_current();

    // One last context switch
    rt.yield_t();

    // If we managed to get this far, the guard has failed for some reason and
    // we should print it as an error. This could be because all tasks have exited.
    // Just in case, we will continue to context switch forever.
    loop {
        eprintln!("guard failed");
        rt.yield_t();
    }
}