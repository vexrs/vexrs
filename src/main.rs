#![no_std]
#![no_main]

use ceros::libv5rt;

extern crate ceros;
extern crate alloc;

use ceros::println;

use ceros::RUNTIME;
use ceros::sync::mutex::Mutex;

lazy_static::lazy_static! {
    static ref GMUTEX: Mutex<u32> = Mutex::new(0);
}

fn task() {
    loop {
        unsafe {
            let _mtx = GMUTEX.acquire();
            println!("Hello, Task!");
            let t = libv5rt::vexSystemTimeGet();
            while libv5rt::vexSystemTimeGet() - t < 1000 {
                RUNTIME.yield_next();
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
            let _mtx = GMUTEX.acquire();
            println!("Hello, Main!");
            let t = libv5rt::vexSystemTimeGet();
            while libv5rt::vexSystemTimeGet() - t < 5000 {
                RUNTIME.yield_next();
            }
            
        }
        RUNTIME.yield_next();
    }
}
