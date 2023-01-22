#![no_std]
#![no_main]



use vexrs::libv5rt;

extern crate vexrs;
extern crate alloc;

static mut PRINT: &[u8] = b"souttest\0";

#[no_mangle]
extern "C" fn main() {

    //vexrs::initialize();


    loop {
        unsafe {
            libv5rt::vexDisplayForegroundColor(0xff0000);
            libv5rt::vexDisplayBigString(0, b"test\0".as_ptr());
            libv5rt::vexSerialWriteBuffer(1, PRINT.as_ptr().cast_mut(), PRINT.len() as u32);
        }
    }
    

    

    
}
