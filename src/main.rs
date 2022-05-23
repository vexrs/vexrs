#![no_std]
#![no_main]

use ceros::libv5rt;

extern crate ceros;


#[no_mangle]
extern "C" fn main() {
    unsafe {
        libv5rt::vexDisplayString(1, b"Hello, World!\0".as_ptr());
    }
}
