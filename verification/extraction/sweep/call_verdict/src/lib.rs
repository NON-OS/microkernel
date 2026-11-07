// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/ipc/call/verdict.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/ipc/call/verdict.rs"]
pub mod verdict;

pub fn wait_ms(caller_ms: u64, default_ms: u64) -> u64 {
    verdict::wait_ms(caller_ms, default_ms)
}

pub fn stale_reply(correlation: u64, want: u64) -> bool {
    verdict::stale_reply(correlation, want)
}

