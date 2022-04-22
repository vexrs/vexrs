
pub fn block(ms: u32) {
    let time = vexv5rt::vexSystemTimeGet();
    while vexv5rt::vexSystemTimeGet() - time < ms {
    }
}