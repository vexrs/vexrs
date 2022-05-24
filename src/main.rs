#![no_std]
#![no_main]

use ceros::libv5rt;

extern crate ceros;

use ceros::RUNTIME;

fn task() {
    loop {
        unsafe {
            libv5rt::vexDisplayString(1, b"Hello, Runtime!\0".as_ptr());
            let t = libv5rt::vexSystemTimeGet();
            while libv5rt::vexSystemTimeGet() - t < 1000 {
                
            }
        }

        RUNTIME.yield_next();
    }
}

#[no_mangle]
extern "C" fn main() {
    RUNTIME.spawn(task);
    loop {
        unsafe {
            libv5rt::vexDisplayString(1, b"Hello, World!\0".as_ptr());
            let t = libv5rt::vexSystemTimeGet();
            while libv5rt::vexSystemTimeGet() - t < 1000 {
                
            }
        }
        RUNTIME.yield_next();
    }
}
