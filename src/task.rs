use core::arch::asm;




/// A context structure for our green threads implementation
#[repr(C)]
struct ThreadContext {
    rsp: u32,
}


pub unsafe fn reset_stack_and_jump(stack: usize, stack_size: usize, f: &dyn Fn()) {
    asm!(
        "mov sp, {s}",
        s = in(reg) stack+stack_size,
    );
    f();
}