// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/ipc/enqueued_at.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/ipc/enqueued_at.rs"]
pub mod enqueued_at;

pub fn enqueued_uptime(uptime_now: u64, stamp_now: u64, stamped: u64) -> u64 {
    enqueued_at::enqueued_uptime(uptime_now, stamp_now, stamped)
}

