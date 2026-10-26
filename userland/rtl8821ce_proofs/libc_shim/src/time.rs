// NONOS Operating System (AGPL-3.0-or-later)

//! The uptime clock as the mounted serve stage reads it: a millisecond on per
//! reading and per idle millisecond, so every wait it runs comes to an end.

use std::cell::Cell;

thread_local! {
    static NOW_MS: Cell<u64> = const { Cell::new(1_000) };
}

/// Milliseconds since boot; each reading moves the clock on by one.
pub fn mk_uptime_ms() -> i64 {
    NOW_MS.with(|t| {
        t.set(t.get() + 1);
        t.get() as i64
    })
}

/// A sleep of `ms`, taken at once: the clock moves on by `ms`.
pub fn mk_idle_ms(ms: u64) -> i64 {
    NOW_MS.with(|t| t.set(t.get() + ms));
    0
}
