# fps_clock
[![https://docs.rs/fps_clock/](https://docs.rs/fps_clock/badge.svg)](https://docs.rs/fps_clock)

A simple crate to control the FPS of your game loops in Rust.


# Usage
This crate is [on crates.io](https://crates.io/crates/fps_clock) and can be
used by adding `fps_clock` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
fps_clock = "1.1"
```

and this to your crate root:

```rust
extern crate fps_clock;
```

To use the FPS clock, just create one with the `FpsClock::new(fps: u32) `method.
Then call the `tick()` method at the end of your game loop.

# Examples

Running your game loop at 30 FPS:

```rust
extern crate fps_clock;
fn main() {
 // Set up your game here
 let mut fps = fps_clock::FpsClock::new(30);
 loop {
     // Complicated game loop stuff here
     fps.tick();
 }
}
```

# License

This crate is licensed under either the MIT or the Apache 2.0 license, depending on what you want. See LICENSE.MIT and LICENSE.APACHE for details.

# Changelog
## v2.0.0
Made `FpsClock::tick()` return the time in nanoseconds since the last time it was called instead of `()`

## v1.0.0
First release.