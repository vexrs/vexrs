
use core::alloc::Layout;

use good_memory_allocator::SpinLockedAllocator;


#[global_allocator]
static HEAP: SpinLockedAllocator = SpinLockedAllocator::empty();


extern "C" {
    static mut _HEAP_SIZE: usize;
    static mut _heap: *mut u8;
}

pub fn initialize_heap() {
    unsafe {
        HEAP.init(_heap as usize, _HEAP_SIZE);
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}