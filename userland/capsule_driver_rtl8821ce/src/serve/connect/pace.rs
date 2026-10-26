// NONOS Operating System (AGPL-3.0-or-later)

//! The connect's waits: the uptime clock, and a millisecond's sleep each time
//! the radio has nothing, so a hunt or a join never spins a core for seconds.

use crate::assoc::JoinClock;

/// The uptime clock, napping while the join hears nothing.
pub(super) struct Uptime;

impl JoinClock for Uptime {
    fn now_ms(&mut self) -> u64 {
        crate::clock::now_ms()
    }
    fn idle(&mut self) {
        nap();
    }
}

/// Sleep a millisecond: the receive ring was empty.
pub(super) fn nap() {
    let _ = nonos_libc::mk_idle_ms(1);
}
