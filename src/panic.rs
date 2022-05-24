// Implements a panic handler for the no_std target

use core::panic::PanicInfo;

/// Called on panic. Just loops for now.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        crate::libv5rt::vexDisplayString(1, alloc::format!("{}", info).as_ptr());
        
    }
    loop {}
}