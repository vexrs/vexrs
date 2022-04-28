

pub fn block(ms: u32) {
    let timer = super::timer::Timer::new(ms);
    timer.block();    
}
