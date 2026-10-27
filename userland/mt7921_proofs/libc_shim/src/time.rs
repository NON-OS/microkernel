// NONOS Operating System (AGPL-3.0-or-later)

//! The uptime clock: a millisecond on per reading and per idle millisecond,
//! so every wait the driver runs comes to an end. `now` reads it without
//! moving it, for the modelled chip's own timing.

use std::cell::Cell;

thread_local! {
    static NOW_MS: Cell<u64> = const { Cell::new(1_000) };
}

pub fn mk_uptime_ms() -> i64 {
    NOW_MS.with(|t| {
        t.set(t.get() + 1);
        t.get() as i64
    })
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    NOW_MS.with(|t| t.set(t.get() + ms));
    0
}

pub fn now() -> u64 {
    NOW_MS.with(Cell::get)
}
