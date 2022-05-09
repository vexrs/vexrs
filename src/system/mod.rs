

use alloc::boxed::Box;

use crate::{hardware::{competition::CompetitionStatus, display::Display, devices::manager::DeviceManager}, runtime::{runner::Runtime, util::get_runtime, MAX_TASKS}};



/// The address of the user program's entry point.
static mut USER_ENTRY: fn() = || {};

/// Runs when the OS task starts in order to initialize the system
pub fn os_init(user_entry: fn()) -> ! {
    
    // Set the global user entry
    unsafe {
        USER_ENTRY = user_entry;
    }

    // Create the global runtime
    let mut runtime = Box::new(Runtime::new());

    // Initialize the runtime
    runtime.init();

    // Create the global display
    let display =  Box::new(Display::new());

    // Initialize it
    display.init();

    // Create a device manager
    let mut device_manager = Box::new(DeviceManager::new());
    
    // Initialize it
    device_manager.init();

    // Spawn the user task
    runtime.spawn(user_entry);


    // Set the current competition status
    let mut comp_status = CompetitionStatus::get_competition_status();
    

    // Begin tick
    loop {
        // Get the runtime
        let rt = get_runtime();


        //------------------------------//
        //   Competition Status Tick    //
        //------------------------------//

        // Get the current competition status
        let new_comp_status = CompetitionStatus::get_competition_status();
        
        // If it is not the same as what we have saved
        // Then we need to update what we have saved
        // and restart the user task
        if new_comp_status != comp_status {
            // Set the global competition status
            comp_status = new_comp_status;

            // Kill all tasks except for the current one.
            for i in 0..MAX_TASKS {
                if i != rt.current_task() {
                    rt.kill_task(i);
                }
            }

            // Start the user task again
            rt.spawn(user_entry);
        }

        //------------------------------//
        //          Draw Tick           //
        //------------------------------//        

        // Clear the screen
        display.clear_screen();

        // Draw to the display
        display.draw();

        //------------------------------//
        //          Tick                //
        //------------------------------//

        // Tick all devices
        device_manager.tick();

        // Tick telemetry
        device_manager.tick_telemetry();

        

        // All loops need to yield
        get_runtime().yield_t();
    }
}
