// NONOS Operating System (AGPL-3.0-or-later)
//! Host stand-in for the uptime syscall. The tests set what it answers, a
//! negative errno included, so the live `after_ms` and `expired` paths run.

use core::sync::atomic::{AtomicI64, Ordering};

static UPTIME: AtomicI64 = AtomicI64::new(0);

pub fn set_uptime(v: i64) {
    UPTIME.store(v, Ordering::SeqCst);
}

pub fn mk_uptime_ms() -> i64 {
    UPTIME.load(Ordering::SeqCst)
}
