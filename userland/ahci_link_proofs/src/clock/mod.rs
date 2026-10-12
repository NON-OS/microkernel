// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's clock on the host: its `Deadline` and its sleep on
//! `std::time::Instant`, with the driver's own waits on top, so the engine
//! files the proofs include time out the way they do in the capsule.

mod host;
#[path = "../../../capsule_driver_ahci/src/clock/wait.rs"]
mod wait;

#[cfg(test)]
pub use host::{idles, now, run_on_model_time};
pub use host::{mk_idle_ms, Deadline};
#[allow(unused_imports)]
pub use wait::{pause_ms, wait_until, Pace};
