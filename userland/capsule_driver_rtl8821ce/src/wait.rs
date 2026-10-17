// NONOS Operating System (AGPL-3.0-or-later)

//! Waits by elapsed time, never by a count of reads (plan/50 C7): a read
//! takes as long as the bus takes, which differs from machine to machine, so
//! a count bounds nothing. Each wait's budget is rtw88's for the same step,
//! and the caller names what did not happen in time.

use crate::clock::now_ms;

/// The deadline `ms` from now. The clock ticks in whole milliseconds, so one
/// more tick: a deadline one tick away could otherwise pass at once.
fn deadline(ms: u64) -> u64 {
    now_ms().saturating_add(ms).saturating_add(1)
}

/// Wait up to `ms` for `ready`, asking once more at the deadline, so a wait
/// that was descheduled past it still sees a late completion. Whether it held.
pub fn until_ms(ms: u64, mut ready: impl FnMut() -> bool) -> bool {
    let end = deadline(ms);
    loop {
        if ready() {
            return true;
        }
        if now_ms() >= end {
            return ready();
        }
        core::hint::spin_loop();
    }
}

/// Wait at least `ms`. A step rtw88 delays by microseconds takes one: the
/// clock is no finer, and longer only settles further.
pub fn delay_ms(ms: u64) {
    let end = deadline(ms.max(1));
    while now_ms() < end {
        core::hint::spin_loop();
    }
}
