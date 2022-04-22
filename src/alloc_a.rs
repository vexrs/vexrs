

// Use this so that crates including ceros-core will automatically use the newlib allocator
use newlib_alloc::Alloc;

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;
