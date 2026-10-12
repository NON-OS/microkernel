// NONOS Operating System (AGPL-3.0-or-later)
//! The uptime clock, in milliseconds. It stands still unless the proof
//! advances it, so a test says exactly how much time a slow device costs.

use std::cell::Cell;

thread_local! {
    static NOW: Cell<u64> = const { Cell::new(0) };
}

/// Move this thread's clock on by `ms`.
pub fn advance(ms: u64) {
    NOW.with(|c| c.set(c.get().saturating_add(ms)));
}

pub fn mk_uptime_ms() -> i64 {
    NOW.with(|c| c.get()) as i64
}

#[derive(Clone, Copy)]
pub struct Deadline {
    end_ms: u64,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end_ms: NOW.with(|c| c.get()).saturating_add(timeout_ms) }
    }

    pub fn expired(&self) -> bool {
        NOW.with(|c| c.get()) >= self.end_ms
    }
}
