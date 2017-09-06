extern crate fps_clock;
use fps_clock::*;
use std::io::Write;
fn main() {
    let mut i = 0.0;
    let mut clock = FpsClock::new(30);
    loop {
        print!("\rTime since last tick (in nanosecs): {:?}", i);
        std::io::stdout().flush().unwrap();
        i = clock.tick();
    }
}
