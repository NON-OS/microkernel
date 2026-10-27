/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The uptime clock every wait is bounded by. A clock that cannot be read
//! reads as far in the future, so a wait on it ends rather than spins.

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

pub fn now_ms() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(u64::MAX)
}

/// Give the CPU away for about `ms`.
pub fn idle_ms(ms: u64) {
    let _ = mk_idle_ms(ms);
}

/// Wait up to `ms` for `ready`, asking once more at the deadline so a wait
/// descheduled past it still sees a late answer. Whether it held.
pub fn until_ms(ms: u64, mut ready: impl FnMut() -> bool) -> bool {
    let end = now_ms().saturating_add(ms);
    loop {
        if ready() {
            return true;
        }
        if now_ms() >= end {
            return ready();
        }
        idle_ms(1);
    }
}

/// Wait at least `ms`.
pub fn delay_ms(ms: u64) {
    let end = now_ms().saturating_add(ms);
    while now_ms() < end {
        idle_ms(1);
    }
}
