

/// A stack guard which makes it impossible to accidentally return to null
#[no_mangle]
pub unsafe extern "C" fn guard() {

    // Panic
    panic!("End of program.")
}