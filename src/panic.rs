// Panic handler for CEROS

use crate::{println, eprintln, runtime::get_runtime};




#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {

    // Print the panic data
    println!("\x07\x1b[1;31mPANIC: {}\x1b[0m", info);
    println!("Panicking Task: {}", get_runtime().current_task());
    eprintln!("\x07\x1b[1;31mPANIC: {}\x1b[0m", info);
    eprintln!("Panicking Task: {}", get_runtime().current_task());

    // Block for a second
    crate::util::block(1000);

    // End our current task
    get_runtime().kill_current();

    loop {}
}
