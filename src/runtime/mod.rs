// The core CEROS runtime support library.

use core::{arch::asm, fmt::Debug};

use crate::{println, print};


pub mod task;
pub mod runner;

use alloc::boxed::Box;
use runner::Runtime;

// A default stack size of 8192 bytes should be plenty
pub const DEFAULT_STACK_SIZE: usize = 0x2000;

// 8 Seems like a good number of tasks. We can increase this if we want.
pub const MAX_TASKS: usize = 8;


// A utility function to get the stack pointer
pub fn get_stack_pointer() -> u32 {
    unsafe {
        let mut sp = 0u32;
        asm!(
            "mov {}, r13",
            out(reg) sp
        );
        sp
    }
}

/// The entry point to the CEROS runtime.
pub fn main(user_entry: fn()) {

    
    
    let mut runtime = Runtime::new();

    // Initialize the runtime
    runtime.init();

    runtime.spawn(user_entry);
    
    println!("ok");

    loop {
        println!("Hello from os task!");
        println!("{}", get_stack_pointer());
        crate::util::block(1000);
        crate::runtime::runner::get_runtime().context_switch();
    }
}

/// This function is the guard for the scheduler. It is called when
/// a function returns and there is nothing above the callstack.
#[no_mangle]
unsafe extern "C"  fn guard() {
    let rt = runner::get_runtime();

    // Kill the current task
    rt.kill_current();

    // One last context switch
    rt.context_switch();

    println!("exit");
    crate::util::block(1000);
    loop {}
}