// NONOS Operating System (AGPL-3.0-or-later)
//! Time on the host. A thread runs on `Instant`, and the driver's sleep only
//! yields, so every wait lasts as long as it would on the target. A thread
//! with a model port plugged in runs on model time instead: a sleep moves
//! its clock on at once, so a 30 s command timeout passes in a moment, and
//! the model's drive keeps to the same clock.

use std::cell::Cell;
use std::time::{Duration, Instant};

thread_local! {
    static AHEAD: Cell<Duration> = const { Cell::new(Duration::ZERO) };
    static MODEL_TIME: Cell<bool> = const { Cell::new(false) };
    static IDLES: Cell<u64> = const { Cell::new(0) };
}

/// Now, on this thread's clock.
pub fn now() -> Instant {
    Instant::now() + AHEAD.with(Cell::get)
}

/// Run this thread on model time while `on`.
#[cfg(test)]
pub fn run_on_model_time(on: bool) {
    MODEL_TIME.with(|m| m.set(on));
}

/// How many times this thread has slept.
#[cfg(test)]
pub fn idles() -> u64 {
    IDLES.with(Cell::get)
}

/// The driver's sleep, `nonos_libc::mk_idle_ms` in the capsule.
pub fn mk_idle_ms(ms: u64) -> i64 {
    IDLES.with(|n| n.set(n.get() + 1));
    if MODEL_TIME.with(Cell::get) {
        AHEAD.with(|a| a.set(a.get() + Duration::from_millis(ms)));
    } else {
        std::thread::yield_now();
    }
    0
}

#[derive(Clone, Copy)]
pub struct Deadline {
    end: Instant,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end: now() + Duration::from_millis(timeout_ms) }
    }

    pub fn expired(&self) -> bool {
        now() >= self.end
    }
}
