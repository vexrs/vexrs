

/// A stack guard which makes it impossible to accidentally return to null
#[no_mangle]
pub unsafe extern "C" fn guard() {
    crate::println!("{}", crate::RUNTIME.current_task());
    // Panic
    panic!("End of program.")
}