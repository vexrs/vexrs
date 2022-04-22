use ceros_serial::{protocol::{DataType, CEROSSerial}, serial::Serial};




#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    
    let mut data = format!("\x07\x1b[1;31mPANIC: {}\x1b[0m\n", info);

    let mut serial_port = Serial::new();
    let mut serial = CEROSSerial::new(&mut serial_port);
    serial.write_data(DataType::Print, data.as_bytes().to_vec());
    
    // Block for 50 ms to wait for the serial data to go through
    crate::util::block(50);

    loop {}
}
