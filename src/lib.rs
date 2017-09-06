// Copyright 2017 by Matthew S.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides a struct for keeping your game loops running at a certian FPS (frames per second)
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/fps_clock) and can be
//! used by adding `fps_clock` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! fps_clock = "0.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust,no_run
//! extern crate fps_clock;
//! ```
//!
//! To use the FPS clock, just create one with the `FpsClock::new(fps: u32) `method.
//! Then call the `tick()` method at the end of your game loop.
//!
//! # Examples
//! Running your game loop at 30 FPS:
//!
//! ``` rust,no_run
//! extern crate fps_clock;
//! fn main() {
//!     // Set up your game here
//!     let mut fps = fps_clock::FpsClock::new(30);
//!     let mut nanosecs_since_last_tick = 0.0;
//!     loop {
//!         // Complicated game loop stuff here
//!         nanosecs_since_last_tick = fps.tick();
//!     }
//! }
//! ```
#![warn(missing_docs)]

use std::time;
use std::thread;

/// A structure that keeps your game loop running at a constant FPS
#[derive(Debug)]
pub struct FpsClock {
    /// The last time the tick() method was called or when the struct was created
    last_tick_time: time::Instant,
    fps: u32,
    fps_in_nanos: f32
}

impl FpsClock {
    /// Constructs a new FpsClock from the specified FPS
    pub fn new(fps: u32) -> FpsClock {
        FpsClock {
            last_tick_time: time::Instant::now(),
            fps: fps,
            fps_in_nanos: (1. / fps as f32) * 1_000_000_000.
        }
    }
    /// Returns the FPS that this FpsClock is set to run at.
    pub fn fps(&self) -> u32 {
        self.fps
    }
    /// Makes the game loop slow down to run at the correct FPS.
    /// Returns the time in nanoseconds since the last time tick() was called.
    pub fn tick(&mut self) -> f32 {
        let t = self.last_tick_time.elapsed();
        let total_nanos = t.as_secs() * 1_000_000_000 + t.subsec_nanos() as u64;
        let diff = self.fps_in_nanos - (total_nanos as f32);
        if diff > 0. {
            thread::sleep(time::Duration::new(0, diff as u32))
        };
        self.last_tick_time = time::Instant::now();
        diff
    }
}
