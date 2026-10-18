// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's clock on the host, kept per test thread: it moves only when
//! the scripted controller spends time on a call or a test moves it, so a
//! wait of minutes runs in microseconds and is measured to the millisecond.

use std::cell::Cell;

#[path = "../../capsule_driver_usb_msc/src/clock/deadline.rs"]
mod deadline;

pub use deadline::Deadline;

thread_local! {
    static NOW: Cell<u64> = const { Cell::new(1_000) };
}

/// The uptime, as the driver reads it.
pub fn now_ms() -> Option<u64> {
    Some(NOW.with(Cell::get))
}

/// Let `ms` pass.
pub fn advance(ms: u64) {
    NOW.with(|c| c.set(c.get().saturating_add(ms)));
}

/// The uptime, for a test to measure with.
pub fn at() -> u64 {
    NOW.with(Cell::get)
}
