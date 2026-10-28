// NONOS Operating System (AGPL-3.0-or-later)
//! The proofs' clock in place of the kernel's uptime: each reading moves it
//! on ten microseconds and each idle by the time asked, so a wait lasts the
//! same number of readings on every machine and a timeout runs at once.

use std::cell::Cell;

thread_local! {
    static MICROS: Cell<u64> = const { Cell::new(0) };
}

/// An idle moves the clock on by the time given.
pub fn idle_ms(ms: u64) {
    MICROS.with(|m| m.set(m.get() + ms * 1000));
}

/// Milliseconds on this thread's clock, which every reading moves on.
pub fn now_ms() -> u64 {
    MICROS.with(|m| {
        let t = m.get() + 10;
        m.set(t);
        t / 1000
    })
}

/// The clock as it stands, without moving it: what the model reads.
pub fn peek_ms() -> u64 {
    MICROS.with(Cell::get) / 1000
}
