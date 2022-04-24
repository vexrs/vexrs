// The core CEROS runtime support library.

use core::arch::asm;

use crate::{println, print};


pub mod task;
pub mod runner;

use alloc::boxed::Box;
use runner::Runtime;

// A default stack size of 8192 bytes should be plenty
pub const DEFAULT_STACK_SIZE: usize = 0x2000;

// 8 Seems like a good number of tasks. We can increase this if we want.
pub const MAX_TASKS: usize = 8;



/// Basic runtime initialization
fn init(user_entry: fn(), main_entry: fn()) {
    // Create a new runtime
    let mut runtime = Box::new(Runtime::new());

    // Spawn the initial user task
    let task = runtime.spawn(user_entry);

    // Jump to the new task's stack
    unsafe {
        asm!(
            "mov lr, {0}",
            "mov sp, {1}",
            "bx {2}",
            in(reg) guard as u32,
            in(reg) task.context.sp,
            in(reg) main_entry as u32,
            
        );
    }
}

/// The entry point to the CEROS runtime.
pub fn main(user_entry: fn()) {
    
    // Initialize the runtime
    // This will not return.
    // It changes to a different stack, where the return
    // point will be a stack guard that hangs forever.
    init(user_entry, || {

        let rt = runner::get_runtime();

        rt.context_switch();
        
    });

    
    
}

/// This function is the guard for the scheduler. It is called when
/// a function returns and there is nothing above the callstack.
#[no_mangle]
unsafe extern "C"  fn guard() {
    println!("exit");
    crate::util::block(1000);
    loop {}
}