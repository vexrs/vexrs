// Use this so that crates including vexrs-core will automatically use the newlib allocator
use newlib_alloc::Alloc;

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;


#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    // Using fmt here increases file size by ~10 Kib !
    panic!("allocation error");
    //panic!("allocation error: {:?}", layout)
}