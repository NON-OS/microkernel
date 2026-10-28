// NONOS Operating System (AGPL-3.0-or-later)

//! The clock every wait reads: the kernel's uptime, in whole milliseconds,
//! and the sleep a wait takes between looks. The proofs mount their own
//! clock in its place.

/// Milliseconds since boot.
pub fn now_ms() -> u64 {
    u64::try_from(nonos_libc::mk_uptime_ms()).unwrap_or(0)
}

/// Give the CPU up for about `ms`: the scheduler parks the capsule.
pub fn idle_ms(ms: u64) {
    let _ = nonos_libc::mk_idle_ms(ms);
}
