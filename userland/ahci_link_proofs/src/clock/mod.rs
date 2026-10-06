// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's clock on the host: its `Deadline` on `std::time::Instant`,
//! with the driver's own waits on top, so the engine files the proofs
//! include time out the way they do in the capsule.

use std::time::{Duration, Instant};

#[path = "../../../capsule_driver_ahci/src/clock/wait.rs"]
mod wait;

#[allow(unused_imports)]
pub use wait::{pause_ms, wait_until};

#[derive(Clone, Copy)]
pub struct Deadline {
    end: Instant,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end: Instant::now() + Duration::from_millis(timeout_ms) }
    }

    pub fn expired(&self) -> bool {
        Instant::now() >= self.end
    }
}
