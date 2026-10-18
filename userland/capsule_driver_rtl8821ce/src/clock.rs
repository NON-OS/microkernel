// NONOS Operating System (AGPL-3.0-or-later)

//! The clock every wait reads: the kernel's uptime, in whole milliseconds.
//! The proofs mount their own clock in its place.

/// Milliseconds since boot.
pub fn now_ms() -> u64 {
    u64::try_from(nonos_libc::mk_uptime_ms()).unwrap_or(0)
}
