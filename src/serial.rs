// Interface to the v5 serial facilities.



use core::prelude::rust_2021::*;
use alloc::vec::Vec;

extern crate acid_io;
extern crate alloc;
use acid_io::{Read, Write};



/// Sends raw data over the serial channel
/// This is only marked as unsafe because it has no checks and should
/// not be used by anything other than a wrapper struct.
unsafe fn send_serial_raw(mut data: Vec<u8>) {
    crate::libv5rt::vexSerialWriteBuffer(1, data.as_mut_ptr(), data.len() as u32);
}


/// Basic serial Read/Write implementation
/// with buffering
#[derive(Default)]
pub struct Serial {
    buffer: Vec<u8>,
}

impl Serial {
    pub fn new() -> Serial {
        Serial { buffer: Vec::new() }
    }
}


impl Read for Serial {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, acid_io::Error> {
        // Read until we have enough bytes
        while self.buffer.len() < buf.len() {

            // Read in the data
            let data = unsafe { crate::libv5rt::vexSerialReadChar(1) };

            // If it is out of bounds exit the loop
            if !(0..=0xff).contains(&data) {
                break;
            }

            self.buffer.push(data as u8);
        }
        
        // Figure how many bytes we have to copy over
        let len = core::cmp::min(buf.len(), self.buffer.len());

        // Drain len bytes from the buffer
        let mut data: Vec<u8> = self.buffer.drain(0..len).collect();
        
        // Make sure the size is buf.len()
        data.resize(buf.len(), 0u8);

        // Copy into buf
        buf.copy_from_slice(&data);

        Ok(len)
    }
}


impl Write for Serial {
    fn write(&mut self, buf: &[u8]) -> Result<usize, acid_io::Error> {

        unsafe {
            send_serial_raw(buf.to_vec());
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), acid_io::Error> {

        

        Ok(())
    }
}



// Println implementations.


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = $crate::serial::Serial::new();
            let mut serial = vexrs_serial::protocol::VexrsSerial::new(&mut serial_port);
            serial.write_data(vexrs_serial::data::DataType::Print(alloc::format!("{}",format_args!($($arg)*)).as_bytes().to_vec()));
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = $crate::serial::Serial::new();
            let mut serial = vexrs_serial::protocol::VexrsSerial::new(&mut serial_port);
            serial.write_data(vexrs_serial::data::DataType::Print(alloc::format!("{}\n",format_args!($($arg)*)).as_bytes().to_vec()));
        }
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = $crate::serial::Serial::new();
            let mut serial = vexrs_serial::protocol::VexrsSerial::new(&mut serial_port);
            serial.write_data(vexrs_serial::data::DataType::Error(alloc::format!("{}",format_args!($($arg)*)).as_bytes().to_vec()));
        }
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut serial_port = $crate::serial::Serial::new();
            let mut serial = vexrs_serial::protocol::VexrsSerial::new(&mut serial_port);
            serial.write_data(vexrs_serial::data::DataType::Error(alloc::format!("{}\n",format_args!($($arg)*)).as_bytes().to_vec()));
        }
    };
}