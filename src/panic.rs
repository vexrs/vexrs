// Implements a panic handler for the no_std target

use core::panic::PanicInfo;

use alloc::string::ToString;

/// Called on panic. Just loops for now.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    
    loop {
        unsafe {
            crate::libv5rt::vexDisplayString(1, info.to_string().as_ptr());
            
        }
    }
}