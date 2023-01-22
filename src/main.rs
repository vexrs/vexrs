#![no_std]
#![no_main]

use core::arch::global_asm;

use vexrs::libv5rt;

extern crate vexrs;
extern crate alloc;

// Include our global assembly code.
global_asm!(include_str!("arm.S"));

#[no_mangle]
extern "C" fn main() {
    loop {
        unsafe {
            
            libv5rt::vexDisplayForegroundColor(0xffff00);
            libv5rt::vexDisplayBigString(0, b"Test\0".as_ptr());
        }
    }
    

    //vexrs::initialize();

    
}
