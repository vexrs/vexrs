
pub fn block(ms: u32) {
    let time = unsafe{vexv5rt::vexSystemTimeGet()};
    while unsafe{vexv5rt::vexSystemTimeGet()} - time < ms {
    }
}