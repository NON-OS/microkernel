// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/core/table/thread_start.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/core/table/thread_start.rs"]
pub mod thread_start;

pub fn start_in_user_half(entry: u64, stack: u64) -> bool {
    thread_start::start_in_user_half(entry, stack)
}

