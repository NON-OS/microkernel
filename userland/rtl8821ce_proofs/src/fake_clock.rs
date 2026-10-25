// NONOS Operating System (AGPL-3.0-or-later)
//! The proofs' clock in place of the kernel's uptime: each reading moves it
//! on ten microseconds, so a wait lasts the same number of readings on every
//! machine and a timeout test runs in no time at all.

use std::cell::Cell;

thread_local! {
    static MICROS: Cell<u64> = const { Cell::new(0) };
}

/// Milliseconds on this thread's clock, which every reading moves on.
pub fn now_ms() -> u64 {
    MICROS.with(|m| {
        let t = m.get() + 10;
        m.set(t);
        t / 1000
    })
}
