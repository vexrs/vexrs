use crate::eprintln;




#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {

    // Print the panic data
    eprintln!("\x07\x1b[1;31mPANIC: {}\x1b[0m", info);
    
    // Block for 50 ms to wait for the serial data to go through
    crate::util::block(50);

    loop {}
}
