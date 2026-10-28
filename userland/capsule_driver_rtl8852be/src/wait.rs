// NONOS Operating System (AGPL-3.0-or-later)

//! Waits by elapsed time, never by a count of reads: a read takes as long as
//! the bus takes, so a count bounds nothing. rtw89 polls with
//! `read_poll_timeout(.., delay_us, timeout_us)`; each wait here takes that
//! timeout rounded up to whole milliseconds (`ms_of`), the clock being no
//! finer, so a step is never given less time than Linux gives it.

use crate::clock::{idle_ms, now_ms};

/// The deadline `ms` from now, one tick further: the clock ticks in whole
/// milliseconds, so a deadline one tick away could otherwise pass at once.
fn deadline(ms: u64) -> u64 {
    now_ms().saturating_add(ms).saturating_add(1)
}

/// rtw89's timeout of `us` microseconds as whole milliseconds, at least 1.
pub const fn ms_of(us: u64) -> u64 {
    let ms = us.div_ceil(1000);
    if ms == 0 {
        1
    } else {
        ms
    }
}

/// Wait up to `ms` for `ready`, asking once more at the deadline so a wait
/// descheduled past it still sees a late completion. Whether it held.
pub fn until_ms(ms: u64, mut ready: impl FnMut() -> bool) -> bool {
    let end = deadline(ms);
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

/// `until_ms` with rtw89's timeout in microseconds.
pub fn until_us(us: u64, ready: impl FnMut() -> bool) -> bool {
    until_ms(ms_of(us), ready)
}

/// Wait at least `ms`. A step rtw89 delays by microseconds takes one.
pub fn delay_ms(ms: u64) {
    let end = deadline(ms.max(1));
    while now_ms() < end {
        idle_ms(1);
    }
}
