// Implements a panic handler for the no_std target

use core::panic::PanicInfo;

/// Called on panic. Just loops for now.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}