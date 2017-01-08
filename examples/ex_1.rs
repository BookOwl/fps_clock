extern crate fps_clock;
use fps_clock::*;
use std::io::Write;
fn main() {
    let mut i = 0;
    let mut clock = FpsClock::new(30);
    loop {
        i += 1;
        print!("\r{:?}", i);
        std::io::stdout().flush().unwrap();
        clock.tick();
    }
}
