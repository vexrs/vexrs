// The core CEROS runtime support library.

use crate::{println, eprintln};


// A basic task structure implementation
pub mod task;

// The actual runtime, including a simple round-robin scheduler.
pub mod runner;

// Utility files for interacting with the runtime
pub mod util;

use alloc::boxed::Box;
use runner::Runtime;

// A default stack size of 8192 bytes should be good.
// This will almost certainly made larger later, but we are using this for ease
// of development, and because this is what 
pub const DEFAULT_STACK_SIZE: usize = 0x2000;

// 8 Seems like a good number of tasks. We can increase this if we want.
pub const MAX_TASKS: usize = 8;


// The global runtime singleton. This is a pointer to the actual runtime
static mut RUNTIME: *const Runtime = 0 as *const Runtime;


/// The entry point to the CEROS runtime.
pub fn main(user_entry: fn()) {
    let mut runtime = Box::new(Runtime::new());

    // Initialize the runtime
    runtime.init();

    runtime.spawn(user_entry);
    
    println!("ok");

    loop {
        println!("Hello from os task!");
        println!("{}", crate::util::get_stack_pointer());
        unsafe {vexv5rt::vexDisplayCenteredString(1, "Hello from os task!!!\0".as_ptr());}
        crate::util::block(20);
        crate::util::get_runtime().yield_for(5000);
        
    }
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