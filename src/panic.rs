// Panic handler for CEROS

use crate::{eprintln, runtime::get_runtime};




#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {

    // Print the panic data
    eprintln!("\x07\x1b[1;31mPANIC: {}\x1b[0m", info);
    eprintln!("Panicking Task: {}", get_runtime().current_task());

    // End our current task
    get_runtime().kill_current();

    loop {}
}
