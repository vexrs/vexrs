// The core CEROS runtime support library.


pub mod task;


// A default stack size of 2MB should be plenty
pub const DEFAULT_STACK_SIZE: usize = 2 * 1024 * 1024;


/// The entry point to the CEROS runtime.
pub fn main(user_entry: fn()) {
    // Just call the user function.
    user_entry();
}