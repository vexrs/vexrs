use super::display::Display;


/// Gets the global display
pub fn get_display() -> &'static mut Display {
    unsafe {
        let disp = super::DISPLAY as *mut Display;
        &mut *disp
    }
}